<template>
  <div class="flex items-center space-x-2 mt-4">
    <button
      class="rounded-full bg-red-700 text-white group-hover:brightness-110 hover:cursor-pointer text-xl flex p-4"
    >
      <fai icon="fa-trash-can" />
    </button>
    <div
      class="flex bg-zinc-800 rounded-lg cursor-pointer transition-all duration-300 hover:brightness-110 mb-4 w-full"
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
                  type="text"
                  name="floating_email"
                  id="floating_email"
                  class="block px-0 w-full text-md text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-teal-500 focus:outline-none focus:ring-0 dark:group-focus-within:border-teal-500 peer cursor-pointer"
                  placeholder="Task Name"
                  required
                  autocomplete="off"
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
                  type="text"
                  name="floating_email"
                  id="floating_email"
                  class="block px-0 w-full text-md text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 focus:outline-none focus:ring-0 dark:group-focus-within:border-teal-500 cursor-pointer"
                  placeholder="Task Description"
                  required
                  autocomplete="off"
                />
              </div>
            </div>
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
