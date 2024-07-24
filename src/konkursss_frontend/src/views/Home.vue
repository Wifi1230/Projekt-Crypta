<template>
  <div class="bg-slate-900 min-h-screen h-full overflow-hidden">
    <main>
      <div class="custom-height"></div>
      <div class="flex justify-center w-full">
        <div class="w-2/3">
          <div class="text-white grid gap-4 my-10">
            <!-- Display posts -->
            <div v-for="(wpis, index) in filteredWpisy" :key="index" class="post drop-shadow-xl bg-gray-700 p-4 relative rounded-lg">
              <div class="flex items-start mb-2">
                <div class="text-sm text-gray-400 mr-4">
                  <span class="block">{{ wpis.username }}</span>
                  <span class="block">{{ wpis.selected_crypto }}</span>
                  <span 
                    class="block" 
                    :class="{
                      'text-green-500': wpis.prediction === 'Bullish',
                      'text-red-500': wpis.prediction === 'Bearish',
                      'text-gray-400': wpis.prediction === 'Info'
                    }"
                  >
                    {{ wpis.prediction }}
                  </span>
                </div>
                <p class="text-white">{{ wpis.post_text }}</p>
              </div>
              <div class="flex justify-between items-center mt-4">
                <button class="bg-red-600 rounded text-white px-4 py-2" @click="deleteWpis(index)">Usuń</button>
                <div class="flex items-center space-x-4">
                  <div class="flex items-center">
                    <img src="/like.png" alt="Like" class="w-6 h-6 cursor-pointer transition-transform transform hover:scale-110" @click="likePost(index)">
                    <span class="ml-2 text-white">{{ wpis.likes }}</span>
                  </div>
                  <div class="flex items-center">
                    <img src="/dislike.png" alt="Dislike" class="w-6 h-6 cursor-pointer transition-transform transform hover:scale-110" @click="dislikePost(index)">
                    <span class="ml-2 text-white">{{ wpis.dislikes }}</span>
                  </div>
                  <div class="flex items-center">
                    <span class="cursor-pointer" @click="toggleComments(index)">⬇️ ({{ wpis.comments.length }})</span>
                  </div>
                </div>
              </div>
              <!-- Comments Section -->
              <div v-if="wpis.showComments" class="mt-4">
                <div class="flex items-start mb-2">
                  <textarea v-model="newCommentText[index]" class="w-full p-2 bg-gray-700 border border-gray-600 rounded text-white" placeholder="Dodaj komentarz..."></textarea>
                  <button @click="addComment(index)" class="bg-blue-600 text-white rounded px-4 py-2 ml-2">Dodaj</button>
                </div>
                <div v-for="(comment, commentIndex) in wpis.comments" :key="comment.id" class="mb-2 p-2 bg-gray-700 border border-slate-900 rounded">
                  <p class="text-white"><strong>{{ comment.username }}:</strong> {{ comment.text }}</p>
                  <div class="flex items-center">
                    <img src="/like.png" alt="Like" class="w-4 h-4 cursor-pointer transition-transform transform hover:scale-110" @click="likeComment(index, commentIndex)">
                    <span class="ml-1 text-white">{{ comment.likes }}</span>
                    <img src="/dislike.png" alt="Dislike" class="w-4 h-4 ml-2 cursor-pointer transition-transform transform hover:scale-110" @click="dislikeComment(index, commentIndex)">
                    <span class="ml-1 text-white">{{ comment.dislikes }}</span>
                  </div>
                </div>
              </div>
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
import { userStore } from '../store'; // upewnij się, że ścieżka jest poprawna


const props = defineProps(['selectedCrypto']);
const userId = ref(''); // Load the user ID appropriately

const wpisy = ref([]);
const filteredWpisy = ref([]);
const newCommentText = ref([]); // To hold the new comment text for each post

// Fetch posts from backend
const pobierzWpisy = async () => {
  const fetchedWpisy = await konkursss_backend.odczytaj_wpisy();
  wpisy.value = fetchedWpisy.map((wpis, index) => ({
    ...wpis,
    showComments: false,
    comments: [],
    index: BigInt(index), // Ensure the index is a BigInt for compatibility
  }));

  await Promise.all(wpisy.value.map(async (wpis, index) => {
    const comments = await konkursss_backend.odczytaj_komentarze(wpis.index);
    wpis.comments = comments;
  }));

  filterPostsByCrypto();
};

// Filter posts by selected cryptocurrency
const filterPostsByCrypto = () => {
  if (props.selectedCrypto && wpisy.value.length > 0) {
    filteredWpisy.value = wpisy.value.filter(wpis => wpis.selected_crypto.toLowerCase() === props.selectedCrypto.toLowerCase());
  } else {
    filteredWpisy.value = [...wpisy.value];
  }
};

// Check if user has liked a post
const userHasLiked = async (postId) => {
  return await konkursss_backend.user_has_liked(userId.value, postId);
};

// Check if user has disliked a post
const userHasDisliked = async (postId) => {
  return await konkursss_backend.user_has_disliked(userId.value, postId);
};

// Handle liking a post
const likePost = async (index) => {
  try {
    const postId = BigInt(index);
    const hasLiked = await userHasLiked(postId);
    const hasDisliked = await userHasDisliked(postId);

    if (hasLiked) {
      // If the user already liked the post, remove the like
      await konkursss_backend.like_wpis(userId.value, postId);
    } else {
      // If the user disliked the post, remove the dislike and add a like
      if (hasDisliked) {
        await konkursss_backend.dislike_wpis(userId.value, postId);
      }
      await konkursss_backend.like_wpis(userId.value, postId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to like post:', e);
  }
};

// Handle disliking a post
const dislikePost = async (index) => {
  try {
    const postId = BigInt(index);
    const hasLiked = await userHasLiked(postId);
    const hasDisliked = await userHasDisliked(postId);

    if (hasDisliked) {
      // If the user already disliked the post, remove the dislike
      await konkursss_backend.dislike_wpis(userId.value, postId);
    } else {
      // If the user liked the post, remove the like and add a dislike
      if (hasLiked) {
        await konkursss_backend.like_wpis(userId.value, postId);
      }
      await konkursss_backend.dislike_wpis(userId.value, postId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to dislike post:', e);
  }
};

// Toggle comments section visibility
const toggleComments = (index) => {
  filteredWpisy.value[index].showComments = !filteredWpisy.value[index].showComments;
};

// Add a new comment to a post
const addComment = async (postIndex) => {
  try {
    const postId = BigInt(postIndex);
    const commentText = newCommentText.value[postIndex];
    
    if (commentText.trim() === '') {
      return;
    }
    if (!userStore.username) {
      console.error('User is not logged in');
      return;
    }
    
    const comment = {
      text: commentText,
      username: userStore.username, // Assign current user's username
      likes: 0,
      dislikes: 0,
      wpis_index: postId,
    };

    await konkursss_backend.dodaj_komentarz(postId, comment);
    newCommentText.value[postIndex] = '';
    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to add comment:', e);
  }
};

// Check if user has liked a comment
const userHasLikedComment = async (postId, commentId) => {
  return await konkursss_backend.user_has_liked_comment(userId.value, postId, commentId);
};

// Check if user has disliked a comment
const userHasDislikedComment = async (postId, commentId) => {
  return await konkursss_backend.user_has_disliked_comment(userId.value, postId, commentId);
};

// Handle liking a comment
const likeComment = async (postIndex, commentIndex) => {
  try {
    const postId = BigInt(postIndex);
    const commentId = BigInt(commentIndex);
    const hasLiked = await userHasLikedComment(postId, commentId);
    const hasDisliked = await userHasDislikedComment(postId, commentId);

    if (hasLiked) {
      // If the user already liked the comment, remove the like
      await konkursss_backend.like_comment(userId.value, postId, commentId);
    } else {
      // If the user disliked the comment, remove the dislike and add a like
      if (hasDisliked) {
        await konkursss_backend.dislike_comment(userId.value, postId, commentId);
      }
      await konkursss_backend.like_comment(userId.value, postId, commentId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to like comment:', e);
  }
};

// Handle disliking a comment
const dislikeComment = async (postIndex, commentIndex) => {
  try {
    const postId = BigInt(postIndex);
    const commentId = BigInt(commentIndex);
    const hasLiked = await userHasLikedComment(postId, commentId);
    const hasDisliked = await userHasDislikedComment(postId, commentId);

    if (hasDisliked) {
      // If the user already disliked the comment, remove the dislike
      await konkursss_backend.dislike_comment(userId.value, postId, commentId);
    } else {
      // If the user liked the comment, remove the like and add a dislike
      if (hasLiked) {
        await konkursss_backend.like_comment(userId.value, postId, commentId);
      }
      await konkursss_backend.dislike_comment(userId.value, postId, commentId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to dislike comment:', e);
  }
};

// Auto-refresh posts every 5 seconds
const startAutoRefresh = () => {
  setInterval(async () => {
    await pobierzWpisy();
  }, 5000); // Refresh every 5 seconds
};

// Initialize on component mount
onMounted(async () => {
  await pobierzWpisy();
  startAutoRefresh();
});
</script>

<style>
.custom-height {
  height: 20vh; /* Adjust the value based on your needs */
}
</style>
