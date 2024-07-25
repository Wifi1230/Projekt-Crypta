<template>
  <div @click="handleClickOutside">
    <header class="bg-gray-700 w-full h-1/5 fixed top-0 z-50">
      <div class="h-3/4 flex justify-center items-center">
        <router-link to="/" class="h-full aspect-square" @click="resetSelection">
          <img src="/logo2.png" alt="CRYPTA" class="h-full aspect-square" />
        </router-link>
      </div>
      <div class="h-1/4 grid grid-cols-3 items-center">
        <div class="grid grid-cols-2 w-full h-full align-middle">
          <div @click.stop class="w-full">
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
          <button rows="1" class="w-full h-full text-white text-center "> <router-link to="/voting">Add crypto</router-link></button>
        </div>
        <div class="text-white text-center w-full h-full align-middle px-4 py-2">
          <router-link to="/upload">Upload post</router-link>
        </div>
        <div class="text-white text-center w-full h-full align-middle px-4 py-2">
          <span v-if="userStore.username">{{ userStore.username }}</span>
          <router-link v-else to="/login">Log in</router-link>
          <button v-if="userStore.username" @click="logout" class="ml-4">Logout</button>
        </div>
      </div>
      <div v-if="showCryptocurrencies" class="width15 bg-gray-800 overflow-y-auto max-h-[9rem] z-50 scrollbar-thin scrollbar-thumb-gray-700 scrollbar-track-gray-800">
        <ul class="text-sm text-white">
          <li v-for="crypto in filteredCryptocurrencies" :key="crypto.id" @click="selectCrypto(crypto)" class="flex items-center h-10 px-4 cursor-pointer">
            <p>{{ crypto.shortcut }} {{ crypto.name }}</p>
          </li>
        </ul>
      </div>
    </header>
    <main>
      <router-view :selectedCrypto="selectedCrypto"></router-view>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { konkursss_backend } from 'declarations/konkursss_backend/index';
import { userStore } from './store';

const cryptocurrencies = ref([]);
const filteredCryptocurrencies = ref([]);
const searchQuery = ref('');
const showCryptocurrencies = ref(false);
const selectedCrypto = ref('');

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
  selectedCrypto.value = crypto.shortcut;
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

const logout = () => {
  userStore.setUsername('');
};

onMounted(async () => {
  await downloadCryptocurrencies();
  startAutoRefresh();
});
</script>

<style scoped>
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
