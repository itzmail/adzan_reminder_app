<!-- tailwindcss:ignore -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type Kota = { id: string; nama: string };

  let cities: Kota[] = $state([]);
  let selectedCity: string = $state("");
  let schedule: any = $state(null);
  let loading = $state(false);
  let loadingSchedule = $state(false);
  let error = $state("");

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  onMount(async () => {
    try {
      cities = await invoke("get_cities");

      const saved = localStorage.getItem("selected_city_id");
      if (saved) {
        selectedCity = saved;
        await loadSchedule(selectedCity);
      }
    } catch (e) {
      error = "Gagal menyimpan kota: " + String(e);
      console.error(e);
    } finally {
      loading = false;
    }
  });

  async function handleSaveCity() {
    if (!selectedCity) return;

    try {
      await invoke("save_selected_city", { cityId: selectedCity });
      localStorage.setItem("selected_city_id", selectedCity);
      await loadSchedule(selectedCity);
    } catch (e) {
      error = "Gagal fetch jadwal: " + String(e);
    } finally {
      loadingSchedule = false;
    }
  }

  async function loadSchedule(cityId: string) {
    loadingSchedule = true;
    try {
      schedule = await invoke("get_today_schedule", { cityId });
    } catch (e) {
      error = "Gagal fetch jadwal" + String(e);
    } finally {
      loadingSchedule = false;
    }
  }
</script>

<main class="p-8 max-w-2xl mx-auto">
  <h1 class="text-3xl font-bold mb-8 text-center">Adzan Reminder Setup</h1>

  {#if loading}
    <p class="text-center">Loading list kota...</p>
  {:else if error}
    <p class="text-red-500 text-center">{error}</p>
  {:else}
    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow">
      <label for="city-select" class="block text-lg font-medium mb-3">
        Pilih Kota / Kabupaten:
      </label>
      <select
        id="city-select"
        bind:value={selectedCity}
        class="w-full p-3 border rounded-lg mb-4"
      >
        <option value="" disabled>Pilih kota...</option>
        {#each cities as city}
          <option value={city.id}>{city.nama}</option>
        {/each}
      </select>

      <button
        onclick={handleSaveCity}
        disabled={!selectedCity || loadingSchedule}
        class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 rounded-lg disabled:opacity-50"
      >
        {#if loadingSchedule}
          Menyimpan & memuat jadwal...
        {:else}
          Simpan Kota & Lihat Jadwal
        {/if}
      </button>
    </div>

    {#if schedule}
      <div class="mt-8 bg-green-50 dark:bg-green-900 p-6 rounded-lg">
        <h2 class="text-xl font-bold mb-4">
          Jadwal Sholat Hari Ini - {schedule.data.lokasi}
        </h2>
        <div class="grid grid-cols-2 md:grid-cols-3 gap-4 text-center">
          <div class="bg-white dark:bg-gray-800 p-4 rounded">
            <p class="text-sm text-gray-600">Subuh</p>
            <p class="text-2xl font-bold">{schedule.data.jadwal.subuh}</p>
          </div>
          <div class="bg-white dark:bg-gray-800 p-4 rounded">
            <p class="text-sm text-gray-600">Dzuhur</p>
            <p class="text-2xl font-bold">{schedule.data.jadwal.dzuhur}</p>
          </div>
          <div class="bg-white dark:bg-gray-800 p-4 rounded">
            <p class="text-sm text-gray-600">Ashar</p>
            <p class="text-2xl font-bold">{schedule.data.jadwal.ashar}</p>
          </div>
          <div class="bg-white dark:bg-gray-800 p-4 rounded">
            <p class="text-sm text-gray-600">Maghrib</p>
            <p class="text-2xl font-bold">{schedule.data.jadwal.maghrib}</p>
          </div>
          <div class="bg-white dark:bg-gray-800 p-4 rounded">
            <p class="text-sm text-gray-600">Isya</p>
            <p class="text-2xl font-bold">{schedule.data.jadwal.isya}</p>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</main>

<style lang="postcss">
  @reference "tailwindcss";
  :global(body) {
    background: #f3f4f6;
    font-family: system-ui, sans-serif;
  }
</style>
