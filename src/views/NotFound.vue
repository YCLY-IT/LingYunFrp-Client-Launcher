<template>
  <div
    class="not-found flex h-full min-h-[60vh] flex-col items-center justify-center gap-4 p-8 text-center"
  >
    <motion.h1
      class="m-0 bg-gradient-to-r from-primary to-primary-hover bg-clip-text text-6xl font-bold text-transparent"
      :initial="{ opacity: 0, scale: 0.8, y: 20 }"
      :animate="{ opacity: 1, scale: 1, y: 0 }"
      :transition="{ type: 'spring', stiffness: 200, damping: 18 }"
    >
      404 Not Found
    </motion.h1>
    <motion.p
      class="m-0 text-[var(--n-text-color-2)]"
      :initial="{ opacity: 0, y: 16 }"
      :animate="{ opacity: 1, y: 0 }"
      :transition="{ delay: 0.12, duration: 0.4, ease: [0.4, 0, 0.2, 1] }"
    >
      The page you are looking for does not exist.
    </motion.p>
    <motion.div
      :initial="{ opacity: 0, y: 16 }"
      :animate="{ opacity: 1, y: 0 }"
      :transition="{ delay: 0.22, duration: 0.4, ease: [0.4, 0, 0.2, 1] }"
    >
      <router-link
        to="/"
        class="inline-block rounded-full bg-primary px-5 py-2 text-white no-underline transition-transform duration-200 hover:scale-105"
      >
        Go to Home
      </router-link>
    </motion.div>
  </div>
</template>
<script setup lang="ts">
import { motion } from "motion-v";
</script>
