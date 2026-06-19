import { redirect } from '@sveltejs/kit';
import { get } from 'svelte/store';
import { loadedSaveFile } from '$lib/store';
import type { LayoutLoad } from './$types';

export const ssr = false;

export const load: LayoutLoad = ({ route }) => {
  if (route.id !== '/select' && !get(loadedSaveFile)) {
    redirect(307, '/select');
  }
};
