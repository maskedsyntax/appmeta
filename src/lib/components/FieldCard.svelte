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
    await navigator.clipboard.writeText(displayValue);
    oncopy?.();
  }

  const value = $derived(("value" in field ? field.value : "") ?? "");
  const displayValue = $derived.by(() => {
    if (value) return value;
    if ("alternatives" in field && field.alternatives?.length) {
      return field.alternatives[0];
    }
    return "";
  });
  const charCount = $derived(
    "character_count" in field && field.value
      ? field.character_count
      : displayValue.length,
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
      <button type="button" onclick={copy} disabled={!displayValue}>Copy</button>
      <button type="button" class="regenerate" onclick={regenerate} disabled={generating}>
        {generating ? "..." : "Regenerate"}
      </button>
    </div>
  </div>
  {#if displayValue}
    <textarea readonly rows={Math.min(12, Math.max(3, displayValue.split("\n").length))}>{displayValue}</textarea>
  {:else}
    <p class="empty">Not generated yet — click Regenerate or use Generate All.</p>
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
    border: 1px solid var(--color-border-subtle);
    border-radius: var(--radius-md);
    padding: 0.85rem 1rem;
    margin-bottom: 0.85rem;
    background: var(--color-surface);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.65rem;
    gap: 0.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--color-border-subtle);
  }

  h3 {
    margin: 0;
    font-size: 0.875rem;
    font-weight: 600;
    text-transform: capitalize;
    color: var(--color-text);
    letter-spacing: -0.01em;
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
    font-size: 0.875rem;
    padding: 0.55rem 0.65rem;
    resize: vertical;
    background: var(--color-bg-input);
    color: var(--color-text);
    border: 1px solid var(--color-border-subtle);
    border-radius: var(--radius-sm);
    line-height: 1.5;
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
    background: transparent;
    border-color: rgba(74, 158, 255, 0.35);
    color: var(--color-primary);
  }

  .regenerate:hover:not(:disabled) {
    background: var(--color-primary-muted);
  }

  button {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
  }
</style>
