<script lang="ts">
  interface Step {
    number: string;
    title: string;
    description: string;
    cmd?: string;
  }

  interface Props {
    steps: Step[];
  }

  let { steps }: Props = $props();
</script>

<ol class="steps">
  {#each steps as step}
    <li class="step">
      <span class="step-num">{step.number}</span>
      <div class="step-body">
        {#if step.cmd}
          <code class="step-cmd">{step.cmd}</code>
        {/if}
        <h3>{step.title}</h3>
        <p class="muted">{step.description}</p>
      </div>
    </li>
  {/each}
</ol>

<style>
  .steps {
    list-style: none;
    padding: 0;
    margin: 0;
    display: grid;
    gap: 0;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .step {
    display: grid;
    grid-template-columns: 3rem 1fr;
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .step:last-child {
    border-bottom: none;
  }

  .step-num {
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 1.25rem;
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--color-text-dim);
    background: var(--color-bg-elevated);
    border-right: 1px solid var(--color-border-subtle);
  }

  .step-body {
    padding: 1.25rem 1.5rem;
    background: var(--color-bg-panel);
  }

  .step-cmd {
    display: block;
    font-size: 0.75rem;
    color: var(--color-accent);
    background: none;
    padding: 0;
    margin-bottom: 0.5rem;
  }

  .step-body h3 {
    margin-bottom: 0.35rem;
  }

  .step-body p {
    margin: 0;
    font-size: 0.9375rem;
  }

  @media (min-width: 768px) {
    .steps {
      grid-template-columns: repeat(5, 1fr);
    }

    .step {
      grid-template-columns: 1fr;
      border-bottom: none;
      border-right: 1px solid var(--color-border-subtle);
    }

    .step:last-child {
      border-right: none;
    }

    .step-num {
      border-right: none;
      border-bottom: 1px solid var(--color-border-subtle);
      padding: 0.75rem;
      justify-content: flex-start;
      padding-left: 1.25rem;
    }

    .step-body {
      padding: 1.25rem;
    }
  }
</style>
