import Clock from "./clock.svelte";
import Album from "./album.svelte";
import Genre from "./genre.svelte";
import Bpm from "./bpm.svelte";
import type { Component } from "svelte";

export { Clock, Album, Genre, Bpm }

export enum Icon {
    Clock,
    Album,
    Genre,
    Bpm,
}

export interface IconProps {
    width?: number | string;
    style?: string;
}

export function iconComponent(icon: Icon): Component<IconProps> {
    switch (icon) {
        case Icon.Clock:
            return Clock;
        case Icon.Album:
            return Album;
        case Icon.Genre:
            return Genre;
        case Icon.Bpm:
            return Bpm;
    }
}
