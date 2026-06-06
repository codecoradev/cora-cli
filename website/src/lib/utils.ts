import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { Snippet } from 'svelte';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

type WithoutChildren<T> = T extends { children?: Snippet } ? Omit<T, 'children'> : T;
type ElementOf<T> = T extends { ref?: infer E } ? E : never;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = WithoutChildren<T> & {
	ref?: U | null;
	children?: Snippet;
};
