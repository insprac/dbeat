<!--
@component

A generic text input type which is a simple wrapper for `<input type="text" />`.
Events don't need to be handled directly, the `onInput` callback property is called with the string
value evey time a change is made to the input.

@example

```svelte
<script lang="ts">
    let searchTerm = $state("");

    function onInput(value: string) {
        searchTerm = value;
    }
</script>

<TextInput value={searchTerm} {onInput} placeholder="Search..." />
```
-->

<script lang="ts">

    let {
        value,
        placeholder,
        onInput,
    }: {
        value?: string;
        placeholder?: string;
        onInput: (value: string) => void;
    } = $props();

    function inputHandler(event: Event) {
        if (event.target) {
            onInput((event.target as HTMLInputElement).value);
        }
    }
</script>

<input type="text" oninput={inputHandler} {placeholder} {value} />

<style>
    input {
        width: 100%;
        padding: 0.4rem 0.8rem;
        background-color: #333;
        border-radius: 4px;
        border: none;
        font-size: 16px;
        color: white;
    }
</style>
