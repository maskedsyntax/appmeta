<script lang="ts">
  import { project, persistProject } from "$lib/stores/appStore";
  import { PRIVACY_FIELDS } from "$lib/types";
  import FieldCard from "$lib/components/FieldCard.svelte";
  import GenerateAllBar from "$lib/components/GenerateAllBar.svelte";

  function getField(name: string) {
    return $project?.generated_fields.find((f) => f.field === name) ?? { field: name, value: "" };
  }
</script>

<h2>Privacy Helper</h2>
<p class="disclaimer">
  This tool generates draft privacy text based on your confirmed answers.
  It does <strong>not</strong> automatically answer Apple privacy labels.
</p>

{#if !$project}
  <p class="empty">No project loaded.</p>
{:else}
  <section class="questionnaire">
    <h3>Privacy Questionnaire</h3>
    <form onsubmit={(e) => { e.preventDefault(); persistProject($project!); }}>
      <label><input type="checkbox" checked={$project.privacy.collects_personal_data === true} onchange={(e) => ($project.privacy.collects_personal_data = e.currentTarget.checked ? true : false)} /> Collects personal data</label>
      <label><input type="checkbox" checked={$project.privacy.uses_analytics === true} onchange={(e) => ($project.privacy.uses_analytics = e.currentTarget.checked ? true : false)} /> Uses analytics</label>
      <label><input type="checkbox" checked={$project.privacy.uses_crash_reporting === true} onchange={(e) => ($project.privacy.uses_crash_reporting = e.currentTarget.checked ? true : false)} /> Uses crash reporting</label>
      <label><input type="checkbox" checked={$project.privacy.uses_ads === true} onchange={(e) => ($project.privacy.uses_ads = e.currentTarget.checked ? true : false)} /> Shows ads</label>
      <label><input type="checkbox" checked={$project.privacy.uses_tracking === true} onchange={(e) => ($project.privacy.uses_tracking = e.currentTarget.checked ? true : false)} /> Uses tracking</label>
      <label><input type="checkbox" checked={$project.privacy.uses_cloud_sync === true} onchange={(e) => ($project.privacy.uses_cloud_sync = e.currentTarget.checked ? true : false)} /> Uses cloud sync</label>
      <label>Privacy Policy URL <input bind:value={$project.privacy.privacy_policy_url} type="url" /></label>
      <label>Notes <textarea bind:value={$project.privacy.notes} rows="3"></textarea></label>
      <button type="submit">Save Privacy Answers</button>
    </form>
  </section>

  <GenerateAllBar fields={[...PRIVACY_FIELDS]} label="Generate All Privacy Fields" />

  {#each PRIVACY_FIELDS as fieldName}
    <FieldCard field={getField(fieldName)} />
  {/each}
{/if}

<style>
  .questionnaire { margin-bottom: 1.5rem; }
  form label { display: block; margin: 0.4rem 0; }
  input[type="url"],
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
