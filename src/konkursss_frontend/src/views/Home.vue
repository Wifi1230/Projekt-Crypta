<template>
  <div class="bg-slate-900  min-h-screen h-full overflow-hidden">
    <!-- Główna zawartość strony -->
    <main>
      <div class="custom-height"></div>
      <!-- Placeholder na główną zawartość -->
      <div class="flex  justify-center w-full">
        <div class="w-2/3">
          <div class="grid gap-4 my-10">
            <!-- Wyświetlanie wpisów -->
            <div v-for="(wpis, index) in wpisy" :key="index" class="drop-shadow-xl bg-gray-700 p-4">
              <p>id: {{ index }}</p>
              <p>{{ wpis }}</p>
              <button class="bg-gray-900 rounded text-white p-4" @click="deleteWpis(index)">usun</button>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { konkursss_backend } from 'declarations/konkursss_backend/index';

// Tablica przechowująca wpisy na stronie
const wpisy = ref([]);
const nowyBlog = ref('');

// Funkcje do zarządzania wpisami
const dodajWpisy = async () => {
  wpisy.value = await konkursss_backend.dodaj_wpis(nowyBlog.value);
  await pobierzWpisy();
};

const deleteWpis = async (index) => {
  await konkursss_backend.usun_wpis(index);
  await pobierzWpisy();
};

const pobierzWpisy = async () => {
  wpisy.value = await konkursss_backend.odczytaj_wpisy();
};

// Funkcja do automatycznego odświeżania wpisów
const startAutoRefresh = () => {
  setInterval(async () => {
    await pobierzWpisy();
  }, 3000);
};

// Po zamontowaniu komponentu uruchamia pobieranie wpisów i automatyczne odświeżanie
onMounted(() => {
  pobierzWpisy();
  startAutoRefresh();
});
</script>

<style >
.custom-height {
  height: 20vh; /* Adjust the value based on your needs */
}
</style>