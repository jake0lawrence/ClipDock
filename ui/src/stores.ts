import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Clip { id:number; text:string; pinned:boolean; ts:number }

export const clips = writable<Clip[]>([]);

export async function refresh() {
  clips.set(await invoke<Clip[]>('get_clips'));
}

export async function toggle(id:number) {
  await invoke('toggle_pin', { id });
  refresh();
}

export async function copyToClipboard(text:string) {
  await invoke('copy_to_clipboard', { text });
}

