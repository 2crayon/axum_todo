updateTodoList();

let newTodoForm = document.querySelector("#new-todo-form");

newTodoForm.addEventListener("submit", async (e) => {
  e.preventDefault();
  let todoContent = e.target.elements.content.value;
  let reqBody = JSON.stringify({ content: todoContent });
  e.target.elements.content.value = "";

  await fetch("/api/todos", {
    method: "POST",
    body: reqBody,
    headers: {
      "Content-Type": "application/json",
    },
  });

  await updateTodoList();
});

function templateTodo(todo) {
  return `
<li class="bg-white shadow-md rounded-md overflow-hidden">
  <div
    class="${
      todo.done ? "bg-blue-100 border-blue-200" : "bg-white border-gray-200"
    } h-[40px] border-b-2 flex justify-between p-2"
  >
    <button
      onclick="toggleTodoDone(${todo.id})"
      class="${
        todo.done
          ? "text-white p-[2px] bg-blue-500"
          : "border-2 border-gray-300 bg-white"
      } h-full aspect-square rounded-md"
    >
    ${
      todo.done
        ? `<svg
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      class=""
    >
      <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
      <g
        id="SVGRepo_tracerCarrier"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></g>
      <g id="SVGRepo_iconCarrier">
        <path
          d="M4 12.6111L8.92308 17.5L20 6.5"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        ></path>
      </g>
    </svg>`
        : ""
    }
    </button>
    <button
      onclick="deleteTodo(${todo.id})"
      class="h-full aspect-square bg-red-500 text-white p-[1px] rounded-md"
    >
      <svg
        viewBox="0 -0.5 25 25"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
        <g
          id="SVGRepo_tracerCarrier"
          stroke-linecap="round"
          stroke-linejoin="round"
        ></g>
        <g id="SVGRepo_iconCarrier">
          <path
            d="M6.96967 16.4697C6.67678 16.7626 6.67678 17.2374 6.96967 17.5303C7.26256 17.8232 7.73744 17.8232 8.03033 17.5303L6.96967 16.4697ZM13.0303 12.5303C13.3232 12.2374 13.3232 11.7626 13.0303 11.4697C12.7374 11.1768 12.2626 11.1768 11.9697 11.4697L13.0303 12.5303ZM11.9697 11.4697C11.6768 11.7626 11.6768 12.2374 11.9697 12.5303C12.2626 12.8232 12.7374 12.8232 13.0303 12.5303L11.9697 11.4697ZM18.0303 7.53033C18.3232 7.23744 18.3232 6.76256 18.0303 6.46967C17.7374 6.17678 17.2626 6.17678 16.9697 6.46967L18.0303 7.53033ZM13.0303 11.4697C12.7374 11.1768 12.2626 11.1768 11.9697 11.4697C11.6768 11.7626 11.6768 12.2374 11.9697 12.5303L13.0303 11.4697ZM16.9697 17.5303C17.2626 17.8232 17.7374 17.8232 18.0303 17.5303C18.3232 17.2374 18.3232 16.7626 18.0303 16.4697L16.9697 17.5303ZM11.9697 12.5303C12.2626 12.8232 12.7374 12.8232 13.0303 12.5303C13.3232 12.2374 13.3232 11.7626 13.0303 11.4697L11.9697 12.5303ZM8.03033 6.46967C7.73744 6.17678 7.26256 6.17678 6.96967 6.46967C6.67678 6.76256 6.67678 7.23744 6.96967 7.53033L8.03033 6.46967ZM8.03033 17.5303L13.0303 12.5303L11.9697 11.4697L6.96967 16.4697L8.03033 17.5303ZM13.0303 12.5303L18.0303 7.53033L16.9697 6.46967L11.9697 11.4697L13.0303 12.5303ZM11.9697 12.5303L16.9697 17.5303L18.0303 16.4697L13.0303 11.4697L11.9697 12.5303ZM13.0303 11.4697L8.03033 6.46967L6.96967 7.53033L11.9697 12.5303L13.0303 11.4697Z"
            fill="currentColor"
          ></path>
        </g>
      </svg>
    </button>
  </div>
  <p class="px-4 py-3 text-gray-600">${todo.content}</p>
</li>
  `;
}

async function updateTodoList() {
  let res = await fetch("/api/todos");
  let todos = await res.json();

  let todoList = document.querySelector("#todo-list");
  todoList.innerHTML = "";
  todos.forEach((todo) => {
    todoList.innerHTML += templateTodo(todo);
  });
}

async function deleteTodo(id) {
  await fetch(`/api/todos/${id}`, {
    method: "DELETE",
  });

  await updateTodoList();
}

async function toggleTodoDone(id) {
  await fetch(`/api/todos/${id}/toggle`, {
    method: "PUT",
  });

  await updateTodoList();
}

async function clearDoneTodos() {
  await fetch(`/api/todos/clear-done`, {
    method: "DELETE",
  });

  await updateTodoList();
}
