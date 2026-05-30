<script lang="ts">
  import type { GeneratedField } from "$lib/types";
  import { requestGenerateField } from "$lib/stores/appStore";

  interface Props {
    field: GeneratedField | { field: string; value?: string };
    oncopy?: () => void;
  }
  let { field, oncopy }: Props = $props();

  let generating = $state(false);

  async function regenerate() {
    generating = true;
    await requestGenerateField(field.field);
    generating = false;
  }

  async function copy() {
    const val = "value" in field ? field.value : "";
    await navigator.clipboard.writeText(val ?? "");
    oncopy?.();
  }

  const value = $derived(("value" in field ? field.value : "") ?? "");
  const charCount = $derived(
    "character_count" in field ? field.character_count : value.length,
  );
  const maxChars = $derived("max_characters" in field ? field.max_characters : null);
  const overLimit = $derived(maxChars != null && charCount > maxChars);
</script>

<div class="field-card">
  <div class="header">
    <h3>{field.field.replace(/_/g, " ")}</h3>
    <div class="actions">
      {#if maxChars != null}
        <span class="count" class:over={overLimit}>{charCount}/{maxChars}</span>
      {:else}
        <span class="count">{charCount} chars</span>
      {/if}
      <button type="button" onclick={copy} disabled={!value}>Copy</button>
      <button type="button" class="regenerate" onclick={regenerate} disabled={generating}>
        {generating ? "..." : "Regenerate"}
      </button>
    </div>
  </div>
  {#if value}
    <textarea readonly rows={Math.min(12, Math.max(3, value.split("\n").length))}>{value}</textarea>
  {:else}
    <p class="empty">Not generated yet.</p>
  {/if}
  {#if "warnings" in field && field.warnings?.length}
    <ul class="meta">
      {#each field.warnings as w}<li class="warn">{w}</li>{/each}
    </ul>
  {/if}
  {#if "missing_info" in field && field.missing_info?.length}
    <ul class="meta">
      {#each field.missing_info as m}<li class="missing">{m}</li>{/each}
    </ul>
  {/if}
  {#if "confidence" in field && field.confidence}
    <span class="confidence">Confidence: {field.confidence}</span>
  {/if}
</div>

<style>
  .field-card {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: 0.75rem;
    margin-bottom: 1rem;
    background: var(--color-surface);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
    gap: 0.5rem;
  }

  h3 {
    margin: 0;
    font-size: 1rem;
    text-transform: capitalize;
    color: var(--color-text);
  }

  .actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex-shrink: 0;
  }

  .count {
    font-size: 0.8rem;
    color: var(--color-text-muted);
  }

  .count.over {
    color: var(--color-accent);
    font-weight: 600;
  }

  textarea {
    width: 100%;
    box-sizing: border-box;
    font-family: inherit;
    font-size: 0.9rem;
    padding: 0.5rem;
    resize: vertical;
    background: var(--color-bg-input);
    color: var(--color-text);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
  }

  .empty {
    color: var(--color-text-dim);
    font-style: italic;
    margin: 0;
  }

  .meta {
    margin: 0.5rem 0 0;
    padding-left: 1.2rem;
    font-size: 0.85rem;
  }

  .warn {
    color: var(--color-warning);
  }

  .missing {
    color: var(--color-primary);
  }

  .confidence {
    font-size: 0.75rem;
    color: var(--color-text-dim);
  }

  .regenerate {
    background: var(--gradient-brand-subtle);
    border-color: rgba(56, 151, 255, 0.3);
    color: var(--color-primary);
  }

  button {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
  }
</style>
