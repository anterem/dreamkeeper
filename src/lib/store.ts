import { writable } from 'svelte/store';
import type { SaveFile } from './bindings';

export const loadedSaveFile = writable<SaveFile | null>(null);
