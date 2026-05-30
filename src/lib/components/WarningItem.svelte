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
    padding: 0.4rem 0.6rem;
    margin: 0.25rem 0;
    border-radius: var(--radius-sm);
    font-size: 0.875rem;
    background: var(--color-bg-elevated);
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
