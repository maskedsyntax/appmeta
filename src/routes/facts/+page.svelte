<script lang="ts">
  import { project, scanResult, persistProject } from "$lib/stores/appStore";
  import * as api from "$lib/api/tauri";

  async function setFactStatus(factId: string, status: string) {
    if (!$project) return;
    const updated = await api.updateFact($project, factId, status);
    await persistProject(updated);
  }

  async function submitAnswer(questionId: string, answer: string) {
    if (!$project || !answer) return;
    const updated = await api.answerQuestion($project, questionId, answer);
    await persistProject(updated);
  }

  async function saveDetails() {
    if (!$project) return;
    $project.project.name = $project.app_identity.app_name;
    await persistProject($project);
  }

  let selectedAnswers = $state<Record<string, string>>({});

  const questions = $derived(
    $project?.scan_questions?.length
      ? $project.scan_questions
      : ($scanResult?.questions ?? []),
  );
</script>

<h2>Facts Review</h2>
<p>Confirm detected facts and review auto-filled project details before generating store copy.</p>

{#if !$project}
  <p class="empty">No project loaded. <a href="/">Connect a project</a> first.</p>
{:else}
  {#if questions.length}
    <section>
      <h3>Confirmation Questions</h3>
      <p class="hint">Answer only when the scanner could not infer something from your project files.</p>
      {#each questions as q}
        <div class="question">
          <p><strong>{q.question}</strong></p>
          <p class="reason">{q.reason}</p>
          {#if $project.question_answers[q.id]}
            <p class="answered">Answered: {$project.question_answers[q.id]}</p>
          {:else}
            <div class="options">
              {#each q.options as opt}
                <button
                  type="button"
                  class:selected={selectedAnswers[q.id] === opt}
                  onclick={() => (selectedAnswers[q.id] = opt)}
                >{opt}</button>
              {/each}
            </div>
            <button
              type="button"
              class="submit"
              disabled={!selectedAnswers[q.id]}
              onclick={() => submitAnswer(q.id, selectedAnswers[q.id])}
            >Confirm</button>
          {/if}
        </div>
      {/each}
    </section>
  {/if}

  <section>
    <h3>Suggested Project Details</h3>
    <p class="hint">Pre-filled from your scan (README, spec, Info.plist, folder name). Edit anything that looks wrong, then save.</p>
    <form
      onsubmit={(e) => {
        e.preventDefault();
        saveDetails();
      }}
    >
      <label>
        App Name
        <input bind:value={$project.app_identity.app_name} />
        <small class="field-hint">Used in all generated App Store copy. Not the pubspec package id.</small>
      </label>
      <label>
        Short Summary
        <textarea bind:value={$project.summary.short_summary} rows="3"></textarea>
      </label>
      <label>
        Long Summary
        <textarea bind:value={$project.summary.long_summary} rows="4"></textarea>
      </label>
      <label>
        Primary Category
        <input bind:value={$project.app_identity.primary_category} placeholder="Utilities" />
        <small class="field-hint">Suggested from app description — match App Store Connect categories.</small>
      </label>
      <label>
        Privacy Policy URL
        <input bind:value={$project.privacy.privacy_policy_url} type="url" placeholder="https://..." />
        <small class="field-hint">Detected from README/privacy.md if present — confirm before submit.</small>
      </label>
      <fieldset>
        <legend>Privacy</legend>
        <label><input type="checkbox" checked={$project.privacy.has_account === true} onchange={(e) => ($project.privacy.has_account = e.currentTarget.checked ? true : null)} /> Has account/login</label>
        <label><input type="checkbox" checked={$project.privacy.uses_analytics === true} onchange={(e) => ($project.privacy.uses_analytics = e.currentTarget.checked ? true : null)} /> Uses analytics</label>
        <label><input type="checkbox" checked={$project.privacy.stores_data_locally === true} onchange={(e) => ($project.privacy.stores_data_locally = e.currentTarget.checked ? true : null)} /> Stores data locally</label>
      </fieldset>
      <fieldset>
        <legend>Review</legend>
        <label><input type="checkbox" checked={$project.review.requires_login === true} onchange={(e) => ($project.review.requires_login = e.currentTarget.checked ? true : null)} /> Requires login</label>
        <label>Demo Username <input bind:value={$project.review.demo_username} /></label>
        <label>Demo Password <input bind:value={$project.review.demo_password} /></label>
      </fieldset>
      <button type="submit" class="primary">Save Project</button>
    </form>
  </section>

  <section>
    <h3>Detected Facts</h3>
    <table>
      <thead>
        <tr>
          <th>Fact</th>
          <th>Source</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each $project.source_facts as fact}
          <tr>
            <td>{fact.fact}</td>
            <td><small>{fact.source_file}</small></td>
            <td><span class="status {fact.status}">{fact.status}</span></td>
            <td>
              <button type="button" onclick={() => setFactStatus(fact.id, "verified")}>Verify</button>
              <button type="button" onclick={() => setFactStatus(fact.id, "rejected")}>Reject</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </section>

  <details>
    <summary>Project Truth File (JSON)</summary>
    <pre>{JSON.stringify($project, null, 2)}</pre>
  </details>
{/if}

<style>
  section { margin-bottom: 2rem; }
  .question { margin-bottom: 1rem; }
  .reason { color: var(--color-text-muted); font-size: 0.85rem; }
  .answered { color: var(--color-success); font-weight: 600; }
  .hint { color: var(--color-text-muted); font-size: 0.875rem; margin: 0 0 0.75rem; line-height: 1.45; }
  .field-hint { display: block; margin-top: 0.3rem; font-size: 0.78rem; color: var(--color-text-dim); font-weight: normal; }
  .options { display: flex; flex-wrap: wrap; gap: 0.5rem; margin: 0.5rem 0; }
  .status { font-size: 0.8rem; padding: 0.1rem 0.4rem; border-radius: 3px; }
  form label { display: block; margin-bottom: 0.75rem; color: var(--color-text); }
  textarea,
  input[type="url"],
  input[type="text"] {
    display: block;
    width: 100%;
    max-width: 600px;
    margin-top: 0.25rem;
    padding: 0.4rem;
    box-sizing: border-box;
  }
  fieldset { margin: 1rem 0; }
  fieldset label { display: block; margin: 0.25rem 0; }
  .primary { margin-top: 0.5rem; }
  pre { font-size: 0.7rem; overflow: auto; max-height: 400px; }
  button { margin-right: 0.25rem; }
</style>
