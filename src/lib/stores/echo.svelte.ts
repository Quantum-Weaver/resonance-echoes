import Database from '@tauri-apps/plugin-sql';
import { browser } from '$app/environment';
import type { Echo } from '$lib/types/types';

let db: Database | null = null;
let echoes = $state<Echo[]>([]);
let totalCount = $state(0);
let loading = $state(false);

function rowToEcho(row: Record<string, unknown>): Echo {
	return {
		id: row.id as string,
		name: row.name as string,
		sense: row.sense as string,
		subcategory: row.subcategory as string,
		emoji: row.emoji as string,
		note: row.note != null ? (row.note as string) : undefined,
		intensity: row.intensity as number,
		timestamp: row.timestamp as number,
		createdAt: row.created_at as number,
	};
}

async function initDB() {
	if (!browser || db) return;
	db = await Database.load('sqlite:echoes.db');
	await loadEchoes();
}

async function loadEchoes(limit = 200, offset = 0) {
	if (!db) return;
	loading = true;
	try {
		const rows = await db.select<Record<string, unknown>[]>(
			'SELECT * FROM echoes ORDER BY timestamp DESC LIMIT $1 OFFSET $2',
			[limit, offset]
		);
		echoes = rows.map(rowToEcho);
		const countRows = await db.select<Record<string, unknown>[]>(
			'SELECT COUNT(*) as count FROM echoes'
		);
		totalCount = ((countRows[0]?.count as number) || 0);
	} finally {
		loading = false;
	}
}

async function addEcho(echo: Omit<Echo, 'id' | 'createdAt'>) {
	if (!db) return;
	const id = crypto.randomUUID();
	const createdAt = Date.now();
	await db.execute(
		'INSERT INTO echoes (id, name, sense, subcategory, emoji, note, intensity, timestamp, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)',
		[id, echo.name, echo.sense, echo.subcategory, echo.emoji, echo.note ?? null, echo.intensity, echo.timestamp, createdAt]
	);
	await loadEchoes();
}

async function getEchoesBySense(senseId: string): Promise<Echo[]> {
	if (!db) return [];
	const rows = await db.select<Record<string, unknown>[]>(
		'SELECT * FROM echoes WHERE sense = $1 ORDER BY timestamp DESC',
		[senseId]
	);
	return rows.map(rowToEcho);
}

async function getEchoesByEmoji(emoji: string): Promise<Echo[]> {
	if (!db) return [];
	const rows = await db.select<Record<string, unknown>[]>(
		'SELECT * FROM echoes WHERE emoji = $1 ORDER BY timestamp DESC',
		[emoji]
	);
	return rows.map(rowToEcho);
}

async function searchEchoes(query: string, limit = 50): Promise<Echo[]> {
	if (!db) return [];
	const pattern = `%${query}%`;
	const rows = await db.select<Record<string, unknown>[]>(
		'SELECT * FROM echoes WHERE name LIKE $1 OR note LIKE $1 ORDER BY timestamp DESC LIMIT $2',
		[pattern, limit]
	);
	return rows.map(rowToEcho);
}

export const echoStore = {
	get echoes() { return echoes; },
	get totalCount() { return totalCount; },
	get loading() { return loading; },
	initDB,
	addEcho,
	loadEchoes,
	getEchoesBySense,
	getEchoesByEmoji,
	searchEchoes,
};
