<script lang="ts">
	import { goto } from '$app/navigation';
	import { echoStore } from '$lib/stores/echo.svelte';
	import { SENSES } from '$lib/data/senses';
	import { EMOJI_DEFS } from '$lib/data/emojis';

	function toLocalISO(date: Date): string {
		const offset = date.getTimezoneOffset() * 60000;
		return new Date(date.getTime() - offset).toISOString().slice(0, 16);
	}

	let name = $state('');
	let selectedSense = $state('other');
	let selectedSubcategory = $state('custom');
	let customSubcategoryText = $state('');
	let selectedEmoji = $state('');
	let note = $state('');
	let intensity = $state(3);
	let useCustomTime = $state(false);
	let customTimestamp = $state(toLocalISO(new Date()));
	let saving = $state(false);
	let saveError = $state('');
	let saveSuccess = $state(false);

	const currentSense = $derived(SENSES.find((s) => s.id === selectedSense));

	function selectSense(senseId: string) {
		selectedSense = senseId;
		const sense = SENSES.find((s) => s.id === senseId);
		selectedSubcategory = sense?.subcategories[0]?.id ?? 'custom';
		customSubcategoryText = '';
	}

	function getFinalSubcategory(): string {
		if (selectedSubcategory === 'custom' && customSubcategoryText.trim()) {
			return customSubcategoryText.trim();
		}
		return selectedSubcategory;
	}

	function getTimestamp(): number {
		if (useCustomTime && customTimestamp) {
			return new Date(customTimestamp).getTime();
		}
		return Date.now();
	}

	async function save() {
		console.log('[save] called');
		if (!name.trim() || saving) {
			console.log('[save] blocked — name empty or already saving', { nameEmpty: !name.trim(), saving });
			return;
		}
		saving = true;
		saveError = '';
		const payload = {
			name: name.trim(),
			sense: selectedSense,
			subcategory: getFinalSubcategory(),
			emoji: selectedEmoji || '✨',
			note: note.trim() || undefined,
			intensity,
			timestamp: getTimestamp()
		};
		console.log('[save] payload:', JSON.stringify(payload));
		try {
			await echoStore.addEcho(payload);
			console.log('[save] addEcho returned — showing confirmation');
			saveSuccess = true;
			await new Promise((r) => setTimeout(r, 900));
			goto('/');
		} catch (e) {
			const msg = e instanceof Error ? e.message : String(e);
			console.error('[save] FAILED:', msg, e);
			saveError = msg;
		} finally {
			saving = false;
		}
	}
</script>

<div class="add-page">
	<header class="add-header">
		<button class="back-btn" onclick={() => goto('/')}>←</button>
		<h1 class="add-title">New Echo</h1>
	</header>

	<div class="form">
		<!-- DB error banner -->
		{#if echoStore.dbError}
			<div class="db-error-banner">
				⚠️ Database not ready: {echoStore.dbError}
			</div>
		{/if}

		<!-- Name -->
		<section class="form-section">
			<input
				type="text"
				bind:value={name}
				placeholder="Name this moment..."
				class="name-input"
				maxlength="120"
			/>
		</section>

		<!-- Sense picker -->
		<section class="form-section">
			<div class="section-label">What sense?</div>
			<div class="sense-scroll">
				{#each SENSES as sense}
					<button
						class="sense-btn"
						class:selected={selectedSense === sense.id}
						onclick={() => selectSense(sense.id)}
					>
						<span class="sense-emoji">{sense.emoji}</span>
						<span class="sense-name">{sense.name}</span>
					</button>
				{/each}
			</div>
		</section>

		<!-- Subcategory chips -->
		{#if currentSense && currentSense.subcategories.length > 0}
			<section class="form-section">
				<div class="section-label">Subcategory</div>
				<div class="chip-row">
					{#each currentSense.subcategories as sub}
						<button
							class="chip"
							class:selected={selectedSubcategory === sub.id}
							onclick={() => (selectedSubcategory = sub.id)}
						>
							{sub.name}
						</button>
					{/each}
				</div>
				{#if selectedSubcategory === 'custom'}
					<input
						type="text"
						bind:value={customSubcategoryText}
						placeholder="Name it..."
						class="custom-sub-input"
						maxlength="40"
					/>
				{/if}
			</section>
		{/if}

		<!-- Emoji grid -->
		<section class="form-section">
			<div class="section-label">Feeling</div>
			<div class="emoji-grid" role="group" aria-label="Select a feeling">
				{#each EMOJI_DEFS as def}
					<button
						class="emoji-btn"
						class:selected={selectedEmoji === def.emoji}
						onclick={() => (selectedEmoji = selectedEmoji === def.emoji ? '' : def.emoji)}
						title={def.label}
						aria-label={def.label}
						aria-pressed={selectedEmoji === def.emoji}
					>
						{def.emoji}
					</button>
				{/each}
			</div>
			{#if selectedEmoji}
				{@const def = EMOJI_DEFS.find((d) => d.emoji === selectedEmoji)}
				{#if def}
					<p class="emoji-def">{def.definition}</p>
				{/if}
			{/if}
		</section>

		<!-- Note -->
		<section class="form-section">
			<div class="section-label">Anything else? <span class="optional">(optional)</span></div>
			<textarea bind:value={note} placeholder="..." class="note-input" rows="3" maxlength="500"></textarea>
		</section>

		<!-- Intensity -->
		<section class="form-section">
			<div class="section-label">How intense? <span class="intensity-val">{intensity}/5</span></div>
			<div class="intensity-row" role="group" aria-label="Intensity level">
				{#each [1, 2, 3, 4, 5] as n}
					<button
						class="intensity-dot"
						class:filled={n <= intensity}
						onclick={() => (intensity = n)}
						aria-label="Intensity {n}"
						aria-pressed={intensity === n}
					></button>
				{/each}
			</div>
		</section>

		<!-- Timestamp -->
		<section class="form-section">
			<div class="section-label">When?</div>
			<div class="time-toggle">
				<button class="time-btn" class:active={!useCustomTime} onclick={() => (useCustomTime = false)}>
					Now
				</button>
				<button class="time-btn" class:active={useCustomTime} onclick={() => (useCustomTime = true)}>
					Custom
				</button>
			</div>
			{#if useCustomTime}
				<input type="datetime-local" bind:value={customTimestamp} class="datetime-input" />
			{/if}
		</section>

		<!-- Actions -->
		<section class="form-section actions">
			<button class="cancel-btn" onclick={() => goto('/')}>Cancel</button>
			<button
				class="save-btn"
				class:success={saveSuccess}
				onclick={save}
				disabled={!name.trim() || saving || !!echoStore.dbError || saveSuccess}
			>
				{saveSuccess ? '✓' : saving ? 'Saving…' : 'Save Echo'}
			</button>
		</section>
		{#if saveError}
			<p class="save-error">{saveError}</p>
		{/if}
	</div>
</div>

<style>
	.add-page {
		min-height: 100%;
		padding-top: env(safe-area-inset-top, 0px);
	}

	.db-error-banner {
		margin: 1rem 0 0;
		padding: 0.75rem 1rem;
		background: color-mix(in srgb, #e74c3c 15%, transparent);
		border: 1px solid #e74c3c;
		border-radius: 8px;
		color: #e74c3c;
		font-size: 0.85rem;
	}

	.save-error {
		margin: 0.5rem 0 0;
		font-size: 0.82rem;
		color: #e74c3c;
		text-align: center;
	}

	.add-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem 0.5rem;
		border-bottom: 1px solid var(--border-color);
	}

	.back-btn {
		background: none;
		border: none;
		color: var(--accent);
		font-size: 1.4rem;
		cursor: pointer;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		line-height: 1;
	}
	.back-btn:hover { background: var(--bg-surface); }

	.add-title {
		font-size: 1.1rem;
		font-weight: 600;
		color: var(--text);
		margin: 0;
	}

	.form {
		padding: 0 1.25rem 2rem;
	}

	.form-section {
		margin-top: 1.5rem;
	}

	.section-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-muted);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin-bottom: 0.6rem;
	}

	.optional {
		font-weight: 400;
		text-transform: none;
		letter-spacing: 0;
	}

	.intensity-val {
		font-weight: 400;
		text-transform: none;
		letter-spacing: 0;
		color: var(--accent);
	}

	/* Name input */
	.name-input {
		width: 100%;
		background: var(--bg-surface);
		border: 1px solid var(--border-color);
		border-radius: 10px;
		color: var(--text);
		font-size: 1.1rem;
		padding: 0.75rem 1rem;
		outline: none;
		box-sizing: border-box;
		transition: border-color 0.15s;
	}
	.name-input:focus { border-color: var(--accent); }
	.name-input::placeholder { color: var(--text-muted); }

	/* Sense scroll */
	.sense-scroll {
		display: flex;
		gap: 0.5rem;
		overflow-x: auto;
		padding-bottom: 0.25rem;
		scrollbar-width: none;
	}
	.sense-scroll::-webkit-scrollbar { display: none; }

	.sense-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem 0.75rem;
		background: var(--bg-surface);
		border: 1.5px solid var(--border-color);
		border-radius: 10px;
		color: var(--text-secondary);
		cursor: pointer;
		white-space: nowrap;
		flex-shrink: 0;
		transition: border-color 0.15s, color 0.15s;
	}
	.sense-btn.selected {
		border-color: var(--accent);
		color: var(--accent);
		background: color-mix(in srgb, var(--accent) 12%, transparent);
	}
	.sense-emoji { font-size: 1.4rem; line-height: 1; }
	.sense-name { font-size: 0.65rem; font-weight: 600; }

	/* Chips */
	.chip-row {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
	}

	.chip {
		padding: 0.3rem 0.75rem;
		background: var(--bg-surface);
		border: 1.5px solid var(--border-color);
		border-radius: 20px;
		color: var(--text-secondary);
		font-size: 0.8rem;
		cursor: pointer;
		transition: border-color 0.15s, color 0.15s;
	}
	.chip.selected {
		border-color: var(--accent);
		color: var(--accent);
		background: color-mix(in srgb, var(--accent) 12%, transparent);
	}

	.custom-sub-input {
		margin-top: 0.5rem;
		width: 100%;
		background: var(--bg-surface);
		border: 1px solid var(--border-color);
		border-radius: 8px;
		color: var(--text);
		font-size: 0.9rem;
		padding: 0.5rem 0.75rem;
		outline: none;
		box-sizing: border-box;
	}
	.custom-sub-input:focus { border-color: var(--accent); }
	.custom-sub-input::placeholder { color: var(--text-muted); }

	/* Emoji grid */
	.emoji-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.5rem;
	}

	.emoji-btn {
		font-size: 1.75rem;
		padding: 0.5rem;
		background: var(--bg-surface);
		border: 1.5px solid var(--border-color);
		border-radius: 10px;
		cursor: pointer;
		text-align: center;
		line-height: 1;
		transition: border-color 0.15s, transform 0.1s;
	}
	.emoji-btn.selected {
		border-color: var(--accent);
		background: color-mix(in srgb, var(--accent) 15%, transparent);
		transform: scale(1.08);
	}
	.emoji-btn:active { transform: scale(0.96); }

	/* Note */
	.note-input {
		width: 100%;
		background: var(--bg-surface);
		border: 1px solid var(--border-color);
		border-radius: 10px;
		color: var(--text);
		font-size: 0.95rem;
		padding: 0.75rem 1rem;
		outline: none;
		resize: none;
		box-sizing: border-box;
		font-family: inherit;
		transition: border-color 0.15s;
	}
	.note-input:focus { border-color: var(--accent); }
	.note-input::placeholder { color: var(--text-muted); }

	/* Intensity */
	.intensity-row {
		display: flex;
		gap: 0.75rem;
		align-items: center;
	}

	.intensity-dot {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		border: 2px solid var(--border-color);
		background: transparent;
		cursor: pointer;
		transition: background 0.15s, border-color 0.15s, transform 0.1s;
	}
	.intensity-dot.filled {
		background: var(--accent);
		border-color: var(--accent);
	}
	.intensity-dot:active { transform: scale(0.9); }

	/* Timestamp */
	.time-toggle {
		display: flex;
		gap: 0.5rem;
	}

	.time-btn {
		padding: 0.4rem 1rem;
		background: var(--bg-surface);
		border: 1.5px solid var(--border-color);
		border-radius: 20px;
		color: var(--text-secondary);
		font-size: 0.85rem;
		cursor: pointer;
		transition: border-color 0.15s, color 0.15s;
	}
	.time-btn.active {
		border-color: var(--accent);
		color: var(--accent);
		background: color-mix(in srgb, var(--accent) 12%, transparent);
	}

	.datetime-input {
		margin-top: 0.5rem;
		width: 100%;
		background: var(--bg-surface);
		border: 1px solid var(--border-color);
		border-radius: 8px;
		color: var(--text);
		font-size: 0.9rem;
		padding: 0.5rem 0.75rem;
		outline: none;
		box-sizing: border-box;
	}
	.datetime-input:focus { border-color: var(--accent); }

	/* Actions */
	.actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 2rem;
	}

	.cancel-btn {
		flex: 1;
		padding: 0.85rem;
		background: var(--bg-surface);
		border: 1.5px solid var(--border-color);
		border-radius: 12px;
		color: var(--text-secondary);
		font-size: 1rem;
		cursor: pointer;
		transition: border-color 0.15s;
	}
	.cancel-btn:hover { border-color: var(--text-muted); }

	.save-btn {
		flex: 2;
		padding: 0.85rem;
		background: var(--accent);
		border: none;
		border-radius: 12px;
		color: #fff;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: opacity 0.15s, transform 0.1s;
	}
	.save-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
	.save-btn:not(:disabled):active { transform: scale(0.98); }
	.save-btn.success {
		background: #27ae60;
		opacity: 1;
	}

	.emoji-def {
		margin: 0.5rem 0 0;
		padding: 0.5rem 0.75rem;
		font-size: 0.82rem;
		color: var(--text-muted);
		line-height: 1.55;
		background: color-mix(in srgb, var(--accent) 6%, transparent);
		border-radius: 8px;
		border-left: 2px solid color-mix(in srgb, var(--accent) 40%, transparent);
		animation: fade-in 0.2s ease;
	}

	@keyframes fade-in {
		from { opacity: 0; transform: translateY(-4px); }
		to   { opacity: 1; transform: translateY(0); }
	}
</style>
