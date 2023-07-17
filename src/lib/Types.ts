import type { ComponentType } from "svelte";

type Component = {
    component: ComponentType;
    props: Record<string, any>;
};

export { type Component };
