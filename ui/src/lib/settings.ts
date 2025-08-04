import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
export interface Settings {
  dark: boolean;
  history: number;
  autostart: boolean;
}
export const settings = writable<Settings>({ dark:false, history:20, autostart:false });
export async function load() { settings.set(await invoke('load_settings')); }
export async function save(s:Settings) { await invoke('save_settings', { new:s }); settings.set(s); }
export async function toggleStart(enable:boolean) { await invoke('set_autostart', { enable }); }
