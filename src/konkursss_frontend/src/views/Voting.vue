<template>
  <div class="bg-slate-900 min-h-screen h-full overflow-hidden text-white flex flex-col">
    <div class="custom-height"></div>

    <div class="w-2/3 mx-auto p-4">
    <!-- Section for adding new cryptocurrency proposal -->
    <div class="my-8">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Input fields for new cryptocurrency proposal -->
        <input 
          v-model="newProposal.shortcut" 
          type="text" 
          placeholder="Shortcut" 
          class="p-2 rounded bg-gray-700 border border-gray-600 text-white w-full"
        />
        <input 
          v-model="newProposal.name" 
          type="text" 
          placeholder="Name" 
          class="p-2 rounded bg-gray-700 border border-gray-600 text-white w-full"
        />
      </div>
      <button 
        @click="addProposal" 
        class="mt-4 bg-gray-700 rounded px-4 py-2 text-white w-full"
      >
        Add proposal
      </button>
    </div>

    <!-- Displaying cryptocurrency proposals -->
    <div v-if="proposals.length === 0" class="text-center text-gray-400">
      No proposals to display.
    </div>
    <div v-else>
      <div class="grid grid-cols-2">
      <div 
        v-for="proposal in proposals" 
        :key="proposal.index"
        class="bg-gray-700 p-4 m-2 rounded drop-shadow-xl"
      >
        <!-- Voting section -->
        <div class="flex justify-between">
          <button 
            @click="likeProposal(proposal.index)" 
            class="bg-slate-900 rounded px-4 py-2 text-white"
          >
            Approve {{ proposal.likes }}
          </button>
          <span class="text-lg text-center items-center">{{ proposal.name }} ({{ proposal.shortcut }})</span>
          <button 
            @click="dislikeProposal(proposal.index)" 
            class="bg-slate-900 rounded px-4 py-2 text-white"
          >
            Reject {{ proposal.dislikes }}
          </button>
        </div>
      </div>
    </div>
  </div>
  </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { konkursss_backend } from 'declarations/konkursss_backend/index';
import { userStore } from '../store';

// State for new cryptocurrency proposal
const newProposal = ref({
  shortcut: '',
  name: ''
});

const proposals = ref([]);

// Fetch proposals from backend
const fetchProposals = async () => {
  try {
    const proposalsData = await konkursss_backend.get_all_proposals();
    proposals.value = proposalsData.map((proposal, index) => ({ ...proposal, index }));
    console.log('Proposals fetched:', proposals.value);
  } catch (error) {
    console.error('Failed to fetch proposals:', error);
  }
};

// Add a new proposal
const addProposal = async () => {
  const {shortcut, name } = newProposal.value;
  if (!shortcut.trim() || !name.trim()) {
    return;
  }

  try {
    await konkursss_backend.propose_crypto({
      shortcut,
      name,
      proposer: userStore.username,
      likes: 0,
      dislikes: 0
    });
    // Clear input fields after submission
    newProposal.value = {shortcut: '', name: '' };
    await fetchProposals(); // Refresh the list
  } catch (error) {
    console.error('Failed to add proposal:', error);
  }
};
const userHasLiked = async (poroposalId) => {
  return await konkursss_backend.user_has_liked_proposal(userStore.username, poroposalId);
};

// Check if user has disliked a post
const userHasDisliked = async (poroposalId) => {
  return await konkursss_backend.user_has_disliked_proposal(userStore.username, poroposalId);
};


// Like a proposal
const likeProposal = async (index) => {
  try {
  const poroposalId = BigInt(index);
  const hasLiked = await userHasLiked(poroposalId);
  const hasDisliked = await userHasDisliked(poroposalId);
  
  if (hasLiked) {
      await konkursss_backend.like_proposal(userStore.username, poroposalId);
    } else {
      // If the user disliked the post, remove the dislike and add a like
      if (hasDisliked) {
        await konkursss_backend.dislike_proposal(userStore.username, poroposalId);
      }
      await konkursss_backend.like_proposal(userStore.username, poroposalId);
    }
    await fetchProposals();
  } catch (error) {
    console.error('Failed to like proposal:', error);
  }
};

// Dislike a proposal
const dislikeProposal = async (index) => {
  try {
  const poroposalId = BigInt(index);
  const hasLiked = await userHasLiked(poroposalId);
  const hasDisliked = await userHasDisliked(poroposalId);
  
  if (hasDisliked) {
      await konkursss_backend.dislike_proposal(userStore.username, poroposalId);
    } else {
      // If the user disliked the post, remove the dislike and add a like
      if (hasLiked) {
        await konkursss_backend.like_proposal(userStore.username, poroposalId);
      }
      await konkursss_backend.dislike_proposal(userStore.username, poroposalId);
    }
    await fetchProposals();
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
