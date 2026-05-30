<script lang="ts">
  import { project } from "$lib/stores/appStore";
  import { REVIEW_FIELDS } from "$lib/types";
  import FieldCard from "$lib/components/FieldCard.svelte";

  function getField(name: string) {
    return $project?.generated_fields.find((f) => f.field === name) ?? { field: name, value: "" };
  }
</script>

<h2>App Review Information</h2>
<p>Generate review notes and release notes for App Store Connect.</p>

{#if !$project}
  <p class="empty">No project loaded.</p>
{:else}
  {#if $project.review.requires_login}
    <div class="notice">
      Login required — ensure demo account is filled in on the <a href="/facts">Facts</a> page.
    </div>
  {/if}

  {#each REVIEW_FIELDS as fieldName}
    <FieldCard field={getField(fieldName)} />
  {/each}

  <section class="manual">
    <h3>Demo Account (manual)</h3>
    <p>Username: {$project.review.demo_username || "—"}</p>
    <p>Password: {$project.review.demo_password ? "••••••" : "—"}</p>
    <p><a href="/facts">Edit on Facts page</a></p>
  </section>
{/if}

<style>
  .manual { margin-top: 1rem; }
</style>
