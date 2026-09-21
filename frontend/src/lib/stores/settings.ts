import { writable } from 'svelte/store';
import { api, type AppSettings } from '$lib/api/client';

export const appSettings = writable<AppSettings>({
  show_recent_notes: true,
  recent_notes_count: 5,
  auto_save: true,
  auto_save_delay_ms: 1500,
  enable_draft_recovery: true
});

let isInitialized = false;

export async function loadAppSettings() {
  try {
    const settings = await api.getSettings();
    appSettings.set(settings);
    isInitialized = true;
  } catch (err) {
    console.error('Failed to load settings:', err);
  }
}

export async function updateAppSettings(newSettings: Partial<AppSettings>) {
  appSettings.update((current) => {
    const updated = { ...current, ...newSettings };
    // Async push to backend
    api.updateSettings(updated).catch((err) => {
      console.error('Failed to update settings:', err);
    });
    return updated;
  });
}
