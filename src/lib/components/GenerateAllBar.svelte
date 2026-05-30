<script lang="ts">
  import { loading, generateAllFields } from "$lib/stores/appStore";

  interface Props {
    fields: string[];
    label?: string;
  }
  let { fields, label = "Generate All" }: Props = $props();

  async function run() {
    await generateAllFields(fields);
  }
</script>

<div class="toolbar">
  <button type="button" class="primary" onclick={run} disabled={$loading}>
    {$loading ? "Generating..." : label}
  </button>
  <span class="hint">{fields.length} fields</span>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1.25rem;
    padding: 0.75rem 0.85rem;
    background: var(--color-surface);
    border: 1px solid var(--color-border-subtle);
    border-radius: var(--radius-md);
  }
  .hint {
    font-size: 0.8125rem;
    color: var(--color-text-dim);
  }
</style>
