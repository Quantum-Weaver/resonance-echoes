import { browser } from '$app/environment';

// Ported from Compass's timer store (2026-07-18) with one deliberate
// difference, per KP's Landscape-board note: "echoes should get a timer like
// compass that can also sound off when done not just silence, since a song
// will not be playing in echoes." Compass fades music and pauses at expiry;
// Echoes has no music, so completion is AUDIBLE by design — a gentle chime
// that repeats a few times until dismissed, with an opt-out toggle.

export type TimerMode = 'sand' | 'breathing' | 'dissolve' | 'flower' | 'metatron' | 'cycle' | 'numeric';

const MODE_ORDER: TimerMode[] = ['sand', 'breathing', 'dissolve', 'flower', 'metatron', 'cycle', 'numeric'];

// Locks to numeric and hides the cycle control when the OS prefers reduced motion.
export const prefersReducedMotion =
	browser && window.matchMedia('(prefers-reduced-motion: reduce)').matches;

const SOUND_KEY = 'resonance-echoes-timer-sound';

let totalSecs = $state(0);
let remainingSecs = $state(0);
let isRunning = $state(false);
let completed = $state(false);
let soundOn = $state(browser ? localStorage.getItem(SOUND_KEY) !== 'off' : true);
let mode = $state<TimerMode>(prefersReducedMotion ? 'numeric' : 'sand');

// Module state (not component-local) so the timer survives navigating away
// from /timer — same reasoning as Compass: a page-local interval would
// orphan on unmount or stack a duplicate on revisit.
let tickInterval: ReturnType<typeof setInterval> | null = null;
let chimeTimeout: ReturnType<typeof setTimeout> | null = null;
let chimeCount = 0;
let audioCtx: AudioContext | null = null;

// Created/resumed inside start() — a user gesture — so the Android WebView
// permits playback later when the timer completes unattended.
function ensureAudio(): AudioContext | null {
	if (!browser || !soundOn) return null;
	try {
		if (!audioCtx) audioCtx = new AudioContext();
		if (audioCtx.state === 'suspended') void audioCtx.resume();
		return audioCtx;
	} catch {
		return null; // no audio available — the visual completion state still shows
	}
}

// A soft three-note rise (C5–E5–G5), each note a short sine bell with a
// gentle attack and long decay. Sensory-friendly on purpose: no buzzer.
function playChime() {
	const ctx = ensureAudio();
	if (!ctx) return;
	const notes = [523.25, 659.25, 783.99];
	notes.forEach((freq, i) => {
		const osc = ctx.createOscillator();
		const gain = ctx.createGain();
		osc.type = 'sine';
		osc.frequency.value = freq;
		const t = ctx.currentTime + i * 0.35;
		gain.gain.setValueAtTime(0, t);
		gain.gain.linearRampToValueAtTime(0.18, t + 0.03);
		gain.gain.exponentialRampToValueAtTime(0.0001, t + 1.4);
		osc.connect(gain);
		gain.connect(ctx.destination);
		osc.start(t);
		osc.stop(t + 1.5);
	});
}

function ringLoop() {
	if (!completed || !soundOn) return;
	chimeCount += 1;
	playChime();
	if (chimeCount < 5) {
		chimeTimeout = setTimeout(ringLoop, 4000);
	}
}

function stopChime() {
	if (chimeTimeout) clearTimeout(chimeTimeout);
	chimeTimeout = null;
	chimeCount = 0;
}

function start(minutes: number) {
	cancel(); // replace rather than stack if one's already running
	ensureAudio(); // unlock audio inside the tap that starts the timer
	totalSecs = minutes * 60;
	remainingSecs = minutes * 60;
	isRunning = true;
	tickInterval = setInterval(() => {
		remainingSecs -= 1;
		if (remainingSecs <= 0) {
			isRunning = false;
			if (tickInterval) clearInterval(tickInterval);
			tickInterval = null;
			totalSecs = 0;
			remainingSecs = 0;
			completed = true;
			ringLoop();
		}
	}, 1000);
}

function dismiss() {
	completed = false;
	stopChime();
}

function cancel() {
	isRunning = false;
	completed = false;
	totalSecs = 0;
	remainingSecs = 0;
	if (tickInterval) clearInterval(tickInterval);
	tickInterval = null;
	stopChime();
}

function setSoundOn(v: boolean) {
	soundOn = v;
	if (browser) localStorage.setItem(SOUND_KEY, v ? 'on' : 'off');
	if (!v) stopChime();
}

function cycleMode() {
	if (prefersReducedMotion) return;
	const idx = MODE_ORDER.indexOf(mode);
	mode = MODE_ORDER[(idx + 1) % MODE_ORDER.length];
}

export const timerStore = {
	get totalSecs() { return totalSecs; },
	get remainingSecs() { return remainingSecs; },
	get isRunning() { return isRunning; },
	get completed() { return completed; },
	get soundOn() { return soundOn; },
	get mode() { return mode; },
	start,
	dismiss,
	cancel,
	setSoundOn,
	cycleMode,
};
