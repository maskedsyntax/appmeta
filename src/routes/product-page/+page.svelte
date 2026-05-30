<script lang="ts">
  import { project } from "$lib/stores/appStore";
  import { PRODUCT_PAGE_FIELDS } from "$lib/types";
  import FieldCard from "$lib/components/FieldCard.svelte";
  import GenerateAllBar from "$lib/components/GenerateAllBar.svelte";

  const fieldMap = $derived(
    Object.fromEntries(
      PRODUCT_PAGE_FIELDS.map((name) => [
        name,
        $project?.generated_fields.find((f) => f.field === name) ?? { field: name, value: "" },
      ]),
    ),
  );
</script>

<h2>Product Page</h2>
<p>Generate App Store product page fields from verified project facts.</p>

{#if !$project}
  <p class="empty">No project loaded. <a href="/">Connect a project</a> and confirm facts first.</p>
{:else}
  <GenerateAllBar fields={[...PRODUCT_PAGE_FIELDS]} label="Generate All Product Page Fields" />
  {#each PRODUCT_PAGE_FIELDS as fieldName}
    <FieldCard field={fieldMap[fieldName]} />
  {/each}
{/if}

<style>
  .empty { color: var(--color-text-muted); }
</style>
