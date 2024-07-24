<template>
  <div class="bg-slate-900 min-h-screen h-full overflow-hidden text-white flex flex-col">
    <div class="custom-height"></div>

    <!-- Section for adding new cryptocurrency proposal -->
    <div class="my-8">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <!-- Input fields for new cryptocurrency proposal -->
        <input 
          v-model="newProposal.icon" 
          type="text" 
          placeholder="Ikona (URL)" 
          class="p-2 rounded bg-gray-700 border border-gray-600 text-white w-full"
        />
        <input 
          v-model="newProposal.shortcut" 
          type="text" 
          placeholder="Skrót" 
          class="p-2 rounded bg-gray-700 border border-gray-600 text-white w-full"
        />
        <input 
          v-model="newProposal.name" 
          type="text" 
          placeholder="Nazwa" 
          class="p-2 rounded bg-gray-700 border border-gray-600 text-white w-full"
        />
      </div>
      <button 
        @click="addProposal" 
        class="mt-4 bg-blue-600 rounded px-4 py-2 text-white w-full"
      >
        Dodaj Propozycję
      </button>
    </div>

    <!-- Displaying cryptocurrency proposals -->
    <div v-if="proposals.length === 0" class="text-center text-gray-400">
      Brak propozycji do wyświetlenia.
    </div>
    <div v-else>
      <div 
        v-for="proposal in proposals" 
        :key="proposal.index"
        class="bg-gray-700 p-4 mb-4 rounded drop-shadow-xl"
      >
        <div class="flex items-center mb-4">
          <!-- Display proposal details -->
          <img :src="proposal.icon" alt="Icon" class="w-10 h-10 mr-4 rounded" />
          <div class="flex-1">
            <div class="flex justify-between items-center">
              <span class="text-lg">{{ proposal.name }} ({{ proposal.shortcut }})</span>
              <div class="flex items-center gap-4">
                <span class="text-sm text-gray-400">{{ proposal.likes }} 👍</span>
                <span class="text-sm text-gray-400">{{ proposal.dislikes }} 👎</span>
              </div>
            </div>
          </div>
        </div>
        <!-- Voting buttons -->
        <div class="flex justify-between">
          <button 
            @click="likeProposal(proposal.index)" 
            class="bg-green-600 rounded px-4 py-2 text-white"
          >
            Głosuj Za
          </button>
          <button 
            @click="dislikeProposal(proposal.index)" 
            class="bg-red-600 rounded px-4 py-2 text-white"
          >
            Głosuj Przeciw
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { konkursss_backend } from 'declarations/konkursss_backend/index';
import { userStore } from '../store'; // upewnij się, że ścieżka jest poprawna

// State for new cryptocurrency proposal
const newProposal = ref({
  icon: '',
  shortcut: '',
  name: ''
});

const proposals = ref([]);

// Fetch proposals from backend
const fetchProposals = async () => {
  try {
    proposals.value = await konkursss_backend.get_all_proposals(); // Update method name
    console.log('Proposals fetched:', proposals.value); // Debugging line
  } catch (error) {
    console.error('Failed to fetch proposals:', error);
  }
};

// Add a new proposal
const addProposal = async () => {
  const { icon, shortcut, name } = newProposal.value;
  if (!icon.trim() || !shortcut.trim() || !name.trim()) {
    return;
  }

  try {
    await konkursss_backend.propose_crypto({
      icon,
      shortcut,
      name,
      proposer: userStore.username,
      likes: 0,
      dislikes: 0
    });
    // Clear input fields after submission
    newProposal.value = { icon: '', shortcut: '', name: '' };
    await fetchProposals(); // Refresh the list
  } catch (error) {
    console.error('Failed to add proposal:', error);
  }
};

// Like a proposal
const likeProposal = async (proposalIndex) => {
  console.log('Proposal Index:', proposalIndex); // Debugging line
  if (proposalIndex === undefined || proposalIndex === null) {
    console.error('Proposal index is undefined or null');
    return;
  }
  try {
    const userId = userStore.username;
    await konkursss_backend.like_proposal(userId, BigInt(proposalIndex));
    await fetchProposals(); // Refresh the list to update likes
  } catch (error) {
    console.error('Failed to like proposal:', error);
  }
};

// Dislike a proposal
const dislikeProposal = async (proposalIndex) => {
  console.log('Proposal Index:', proposalIndex); // Debugging line
  if (proposalIndex === undefined || proposalIndex === null) {
    console.error('Proposal index is undefined or null');
    return;
  }
  try {
    const userId = userStore.username;
    await konkursss_backend.dislike_proposal(userId, BigInt(proposalIndex));
    await fetchProposals(); // Refresh the list to update dislikes
  } catch (error) {
    console.error('Failed to dislike proposal:', error);
  }
};

// Fetch proposals on component mount
onMounted(fetchProposals);
</script>

<style>
.custom-height {
  height: 20vh; /* Adjust the value based on your needs */
}
</style>
