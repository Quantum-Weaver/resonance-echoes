// Theme customization
export interface ThemeConfig {
  mode: 'dark' | 'light' | 'amoled';
  accentColor: string;
  presetName?: string;
  fontSize: 'small' | 'medium' | 'large';
}

// Echo — a single journal entry
export interface Echo {
  id: string;
  body: string;
  senseId: string;
  subcategoryId: string | null;
  emoji: string | null;
  intensity: number;
  createdAt: number;
  updatedAt: number;
}

// Sense — top-level perception category (Seen, Heard, Felt, Thought, etc.)
export interface Sense {
  id: string;
  name: string;
  emoji: string;
  description: string;
}

// Subcategory — fine-grain entry under each Sense
export interface Subcategory {
  id: string;
  senseId: string;
  name: string;
  description: string;
}

// Emoji definition — the sensory lexicon atom
export interface EmojiDefinition {
  emoji: string;
  label: string;
  category: string;
  keywords: string[];
  color?: string;
  sound?: string;
  texture?: string;
  temperature?: string;
  definition?: string;
}
