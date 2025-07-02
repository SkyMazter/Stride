<template>
    <nav>
        <h1 @click="returnHome">Stride</h1>
    </nav>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();

const returnHome = () => router.push("/");
const handleClick = () => {
    router.push("/submit");
};
</script>

<style scoped>
nav {
    /* border-right: solid var(--greyscale-800) 0.5px; */
    padding-left: 1rem;
    padding-right: 1rem;
    display: flex;
    flex-flow: column;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    background-color: var(--greyscale-200);
}

a {
    color: var(--greyscale-900);
}
</style>
