<template>
  <div
    class="flex bg-zinc-800 mb-1 rounded-lg cursor-pointer transition-all duration-300 hover:brightness-110"
  >
    <div class="w-[10px] bg-teal-500 rounded-l-lg py-10"></div>
    <div class="flex-grow flex">
      <div class="flex-grow p-3">
        <span class="flex mb-2 group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-teal-500"
          >
            <fai icon="fa-list" />
          </div>
          <div class="flex-grow">
            <div class="relative">
              <div
                class="absolute inset-y-0 end-0 flex items-center pr-3.5 pointer-events-none dark:group-focus-within:text-teal-500"
              >
                <fai icon="fa-pen" />
              </div>
              <input
                type="email"
                name="floating_email"
                id="floating_email"
                class="block px-0 w-full text-md text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-teal-500 focus:outline-none focus:ring-0 dark:group-focus-within:border-teal-500 peer"
                placeholder="Template Name"
                required
              />
            </div>
          </div>
        </span>
        <span class="flex mb-2 group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-teal-500"
          >
            <fai icon="fa-circle-info" />
          </div>
          <div class="flex-grow">
            <div class="relative">
              <div
                class="absolute inset-y-0 end-0 flex items-center pr-3.5 pointer-events-none dark:group-focus-within:text-teal-500"
              >
                <fai icon="fa-pen" />
              </div>
              <input
                type="email"
                name="floating_email"
                id="floating_email"
                class="block px-0 w-full text-md text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 focus:outline-none focus:ring-0 dark:group-focus-within:border-teal-500"
                placeholder="Template Description"
                required
              />
            </div>
          </div>
        </span>
        <span class="flex group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-teal-500"
          ></div>
          <div class="flex-grow">
            <label class="inline-flex items-center cursor-pointer mt-5">
              <input type="checkbox" value="" class="sr-only peer" />
              <div
                class="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-0 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:w-5 after:h-5 after:transition-all dark:border-gray-600 peer-checked:bg-teal-600 dark:peer-checked:bg-teal-600"
              ></div>
              <span class="ms-3 text-sm font-medium dark:text-zinc-400"
                >Private</span
              >
            </label>
          </div>
        </span>
      </div>
    </div>
  </div>
</template>
