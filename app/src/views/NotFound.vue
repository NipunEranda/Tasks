<template>
    <div class="grid place-items-center items-center justify-center h-screen">
        <div>
            <div>
                <uppercase class="text-6xl">Page Not Found</uppercase>
            </div>
            <div class="text-center mt-7">
                <button type="button"
                    class="dark:text-white dark:bg-teal-800 dark:hover:bg-teal-700 focus:outline-none font-medium rounded-lg px-6 py-2.5 text-center inline-flex items-center ring-0 cursor-pointer text-md uppercase"
                    @click="$router.push('/dashboard')">
                    <fai icon="fa-arrow-left" class="mr-2"/>
                    Go Back
                </button>
            </div>
        </div>
    </div>
</template>