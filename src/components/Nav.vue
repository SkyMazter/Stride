<template>
    <nav>
        <h1 @click="returnHome">Stride</h1>
        <span>
            <button class="btn-primary" @click="handleClick">Add New +</button>
        </span>
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
    border-bottom: solid var(--greyscale-700) 0.5px;
    margin-left: 1rem;
    margin-right: 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

a {
    color: var(--greyscale-900);
}
</style>
