

// Theme customization
export interface ThemeConfig {
  mode: 'dark' | 'light' | 'amoled';
  accentColor: string;
  presetName?: string;
  fontSize: 'small' | 'medium' | 'large';
  albumArtShape: 'square' | 'rounded' | 'circular';
}

// Track metadata
export interface Track {
  id: string;
  uri: string;
  filename: string;
  title: string;
  artist: string;
  album: string;
  genre?: string;
  year?: number;
  trackNumber?: number;
  duration: number;
  coverArt?: string;
  dateAdded: number;
  fileSize: number;
  bitrate: number;
  sampleRate: number;
  format: string;
  lastPlayed: number | null;
  playCount: number;
  skipCount: number;
  discNumber?: number;
  lyrics?: string;
  moodEvents?: MoodEvent[];
}

// Album grouping
export interface Album {
  id: string;
  name: string;
  artist: string;
  tracks: Track[];
  coverArt?: string;
  year?: number;
  genre?: string;
}

// Artist grouping
export interface Artist {
  id: string;
  name: string;
  albums: Album[];
  trackCount: number;
}

// Genre grouping
export interface Genre {
  id: string;
  name: string;
  artists: Artist[];
  albums: Album[];
  trackCount: number;
}

// Playlist
export interface Playlist {
  id: string;
  name: string;
  trackIds: string[];
  createdAt: number;
  updatedAt: number;
}

// Mood event (emoji tagging, skip prompts, manual)
export interface MoodEvent {
  id: number;
  trackId: string;
  emoji: string;
  timestamp: number;
  intensity: number;
  comment?: string;
  context: 'manual' | 'skip_prompt' | 'track_end' | 'favorite';
  tags?: string[];
}

