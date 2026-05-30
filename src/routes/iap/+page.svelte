<script lang="ts">
  import { project, persistProject } from "$lib/stores/appStore";
  import { IAP_FIELDS } from "$lib/types";
  import FieldCard from "$lib/components/FieldCard.svelte";

  function getField(name: string) {
    return $project?.generated_fields.find((f) => f.field === name) ?? { field: name, value: "" };
  }
</script>

<h2>In-App Purchases</h2>
<p>Generate IAP metadata and reviewer instructions when your app has in-app purchases.</p>

{#if !$project}
  <p class="empty">No project loaded.</p>
{:else}
  {#if $project.monetization.has_iap !== true}
    <div class="notice">
      No IAP detected yet. If your app has purchases, confirm on the <a href="/facts">Facts</a> page.
    </div>
  {/if}

  <section class="manual">
    <h3>IAP Details</h3>
    <form onsubmit={(e) => { e.preventDefault(); persistProject($project!); }}>
      <label><input type="checkbox" checked={$project.monetization.has_iap === true} onchange={(e) => ($project.monetization.has_iap = e.currentTarget.checked ? true : false)} /> Has in-app purchases</label>
      <label>IAP Type <input bind:value={$project.monetization.iap_type} placeholder="Subscription, one-time unlock, tip jar..." /></label>
      <label><input type="checkbox" bind:checked={$project.monetization.subscriptions} /> Has subscriptions</label>
      <label>Pricing Notes <textarea bind:value={$project.monetization.pricing_notes} rows="2"></textarea></label>
      <button type="submit">Save IAP Details</button>
    </form>
  </section>

  {#each IAP_FIELDS as fieldName}
    <FieldCard field={getField(fieldName)} />
  {/each}
{/if}

<style>
  .notice { margin-bottom: 1rem; }
  .manual { margin-bottom: 1.5rem; }
  form label { display: block; margin: 0.4rem 0; }
  textarea {
    display: block;
    width: 100%;
    max-width: 500px;
    margin-top: 0.25rem;
    padding: 0.4rem;
    box-sizing: border-box;
  }
  button { margin-top: 0.5rem; }
</style>
