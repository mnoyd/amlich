import { writable } from 'svelte/store';

export const selectedDate = writable<Date>(new Date());
export const activeWorkspace = writable<string>('day_console');

export interface UserProfile {
    birthYear?: number;
    birthMonth?: number;
    birthDay?: number;
    birthHour?: number;
    birthMinute?: number;
    gender?: string;
}

export const userProfile = writable<UserProfile>({});
