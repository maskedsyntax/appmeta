<script lang="ts">
  import { settings, saveSettingsStore, resetContextPreviewAck } from "$lib/stores/appStore";
  import { testAiConnection, maskApiKey } from "$lib/api/tauri";
  import type { AppSettings } from "$lib/types";

  let local = $state<AppSettings | null>(null);
  let maskedKey = $state("");
  let testResult = $state("");
  let testing = $state(false);

  $effect(() => {
    if ($settings) local = { ...$settings };
  });

  async function save() {
    if (!local) return;
    await saveSettingsStore(local);
  }

  async function test() {
    if (!local) return;
    await saveSettingsStore(local);
    testing = true;
    testResult = "";
    try {
      testResult = await testAiConnection();
    } catch (e) {
      testResult = String(e);
    } finally {
      testing = false;
    }
  }

  async function showMasked() {
    if (local?.api_key) {
      maskedKey = await maskApiKey(local.api_key);
    }
  }
</script>

<h2>Settings</h2>

<div class="warning-box">
  API keys are stored locally in plain text for MVP. Do not share your config file.
  Secure Keychain storage is planned for a future release.
</div>

{#if local}
  <form onsubmit={(e) => { e.preventDefault(); save(); }}>
    <label>
      Provider
      <select bind:value={local.provider}>
        <option value="openai">OpenAI</option>
        <option value="openrouter">OpenRouter</option>
      </select>
    </label>
    <label>
      API Key
      <input type="password" bind:value={local.api_key} placeholder="sk-..." />
      <button type="button" class="small" onclick={showMasked}>Show masked</button>
      {#if maskedKey}<span class="masked">{maskedKey}</span>{/if}
    </label>
    <label>
      Model
      <input bind:value={local.model} placeholder="gpt-4o-mini" />
      <small class="hint">gpt-4o-mini works well. GPT-5/o1/o3 use different API params (handled automatically).</small>
    </label>
    <label>
      Base URL
      <input bind:value={local.base_url} placeholder="https://api.openai.com/v1" />
    </label>
    <label>
      Temperature
      <input type="number" bind:value={local.temperature} min="0" max="2" step="0.1" />
    </label>
    <label>
      Max Tokens
      <input type="number" bind:value={local.max_tokens} min="256" max="8192" />
    </label>
    <label>
      Tone
      <select bind:value={local.tone}>
        <option value="professional">Professional</option>
        <option value="friendly">Friendly</option>
        <option value="concise">Concise</option>
      </select>
    </label>
    <label class="checkbox-row">
      <input type="checkbox" bind:checked={local.context_preview_acknowledged} />
      Skip AI context preview before generating (already acknowledged)
    </label>
    <div class="actions">
      <button type="submit" class="primary">Save Settings</button>
      <button type="button" onclick={test} disabled={testing || !local.api_key}>
        {testing ? "Testing..." : "Test Connection"}
      </button>
      <button type="button" onclick={resetContextPreviewAck}>Reset Context Preview</button>
    </div>
  </form>

  {#if testResult}
    <div class="test-result" class:ok={testResult === "OK" || testResult.startsWith("OK")}>
      {testResult}
    </div>
  {/if}
{:else}
  <p>Loading settings...</p>
{/if}

<style>
  .warning-box { margin-bottom: 1.5rem; font-size: 0.9rem; }
  form label { display: block; margin-bottom: 1rem; max-width: 480px; }
  input,
  select {
    display: block;
    width: 100%;
    margin-top: 0.25rem;
    padding: 0.4rem;
    box-sizing: border-box;
  }
  .actions { display: flex; gap: 0.5rem; margin-top: 1rem; flex-wrap: wrap; }
  .checkbox-row { display: flex; align-items: center; gap: 0.5rem; font-size: 0.9rem; }
  .checkbox-row input { width: auto; margin: 0; }
  .small { display: inline; width: auto; margin-left: 0.5rem; font-size: 0.8rem; }
  .masked { font-family: monospace; font-size: 0.85rem; color: var(--color-text-muted); margin-left: 0.5rem; }
  .test-result { margin-top: 1rem; padding: 0.75rem; border-radius: var(--radius-sm); }
  .hint { display: block; margin-top: 0.35rem; font-size: 0.8rem; color: var(--color-text-dim); font-weight: normal; }
</style>
