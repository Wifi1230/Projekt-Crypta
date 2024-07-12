<template>
  <div @click="handleClickOutside">
    <header class="bg-gray-700 w-full h-1/5 fixed top-0 z-50">
      <div class="h-3/4 flex justify-center items-center">
        <router-link to="/" class="h-full aspect-square" @click="resetSelection">
          <img src="/logo2.png" alt="CRYPTA" class="h-full aspect-square" />
        </router-link>
      </div>

      <div class="h-1/4 grid grid-cols-3 items-center">
        <div class="grid grid-cols-[1fr_auto] w-full h-full align-middle">
          <div @click.stop>
            <input
              id="crypto_name"
              v-model="searchQuery"
              @input="filterCryptocurrencies"
              @click="toggleCryptocurrencies"
              placeholder="Crypto name"
              rows="1"
              class="text-center placeholder-text-center placeholder-white text-white bg-transparent w-full h-full px-4 py-2 border-none focus:outline-none"
            />
          </div>
          <button class="aspect-square w-fit h-full"></button>
        </div>
        <div class="text-white text-center w-full h-full align-middle px-4 py-2">
          <router-link to="/upload">Upload post</router-link>
        </div>
        <div class="text-white text-center w-full h-full align-middle px-4 py-2">
          <router-link to="/login">Log in</router-link>
        </div>
      </div>

      <!-- Lista filtrowanych kryptowalut z przewijaniem -->
      <div v-if="showCryptocurrencies" class="width15 bg-gray-800 overflow-y-auto max-h-[9rem] z-50 scrollbar-thin scrollbar-thumb-gray-700 scrollbar-track-gray-800">
        <!-- Wyświetlanie kryptowalut -->
        <ul class="text-sm text-white">
          <li v-for="crypto in filteredCryptocurrencies" :key="crypto.id" @click="selectCrypto(crypto)" class="flex items-center h-10 px-4 cursor-pointer">
            <img :src="crypto.icon" alt="Cryptocurrency" class="h-8 mr-2">
            <p>{{ crypto.shortcut }} {{ crypto.name }}</p>
          </li>
        </ul>
      </div>
    </header>
    <main>
      <!-- Przekazujemy selectedCrypto jako props do komponentu renderowanego przez router-view -->
      <router-view :selectedCrypto="selectedCrypto"></router-view>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { konkursss_backend } from 'declarations/konkursss_backend/index';
// Lista kryptowalut
const cryptocurrencies = ref([]);
const filteredCryptocurrencies = ref([]);
const searchQuery = ref('');
const showCryptocurrencies = ref(false);
const selectedCrypto = ref('');

// Pobieranie kryptowalut i filtrowanie
const downloadCryptocurrencies = async () => {
  cryptocurrencies.value = await konkursss_backend.get_all_cryptos();
  filteredCryptocurrencies.value = [...cryptocurrencies.value];
};

const toggleCryptocurrencies = () => {
  showCryptocurrencies.value = !showCryptocurrencies.value;
};

const filterCryptocurrencies = () => {
  const query = searchQuery.value.toLowerCase().trim();
  filteredCryptocurrencies.value = cryptocurrencies.value.filter(crypto =>
    crypto.name.toLowerCase().startsWith(query) || crypto.shortcut.toLowerCase().startsWith(query)
  );
};

const handleClickOutside = () => {
  showCryptocurrencies.value = false;
};

const selectCrypto = (crypto) => {
  selectedCrypto.value = crypto.shortcut; // Przypisz skrót kryptowaluty
  showCryptocurrencies.value = false;
};
const resetSelection = () => {
  selectedCrypto.value = '';
  filteredCryptocurrencies.value = [...cryptocurrencies.value];
};

const startAutoRefresh = () => {
  setInterval(async () => {
    await downloadCryptocurrencies();
  }, 15000);
};

onMounted(async () => {
  await downloadCryptocurrencies();
  startAutoRefresh();
});
</script>

<style scoped>
/* scrollbar style */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-thumb {
  background-color: rgba(55, 65, 81, 1);
  border-radius: 4px;
}

::-webkit-scrollbar-track {
  background-color: rgba(75, 85, 101, 1);
}

.scrollbar-thin {
  scrollbar-width: thin;
  scrollbar-color: rgba(55, 65, 81, 1) rgba(75, 85, 101, 1);
}

.scrollbar-thin:hover {
  scrollbar-color: rgba(35, 45, 61, 1) rgba(55, 65, 81, 1);
}

ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}

li {
  cursor: pointer;
}

.width15 {
  width: 15%;
}
</style>