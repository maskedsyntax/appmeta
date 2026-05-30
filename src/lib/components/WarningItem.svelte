<script lang="ts">
  interface Props {
    severity: string;
    message: string;
    field?: string;
  }
  let { severity, message, field = "" }: Props = $props();
</script>

<div
  class="warning"
  class:error={severity === "error"}
  class:warn={severity === "warning"}
  class:info={severity === "info"}
>
  {#if field}<span class="field">{field}:</span>{/if}
  {message}
</div>

<style>
  .warning {
    padding: 0.5rem 0.65rem;
    margin: 0 0 0.35rem;
    border-radius: var(--radius-sm);
    font-size: 0.8125rem;
    line-height: 1.45;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-subtle);
    border-left: 3px solid var(--color-text-dim);
    color: var(--color-text-muted);
  }
  .error {
    background: var(--color-error-bg);
    border-color: var(--color-error);
    color: var(--color-error);
  }
  .warn {
    background: var(--color-warning-bg);
    border-color: var(--color-warning);
    color: var(--color-warning);
  }
  .info {
    background: var(--color-info-bg);
    border-color: var(--color-info);
    color: var(--color-primary);
  }
  .field {
    font-weight: 600;
    margin-right: 0.25rem;
  }
</style>
