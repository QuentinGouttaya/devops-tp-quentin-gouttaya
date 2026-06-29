<script>
  let tasks = $state([]);
  let newTitle = $state('');
  let newType = $state('chore');
  const API = import.meta.env.VITE_API_URL || 'http://localhost:3000/tasks';

  $effect(() => {
    fetchTasks();
  });

  async function fetchTasks() {
    const res = await fetch(API);
    tasks = await res.json();
  }

  async function addTask(e) {
    e.preventDefault();
    if (!newTitle) return;
    await fetch(API, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        title: newTitle,
        task_type: newType,
        description: null,
      }),
    });
    newTitle = '';
    newType = 'chore';
    fetchTasks();
  }

  async function toggleTask(id, completed) {
    await fetch(`${API}/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ completed: !completed }),
    });
    fetchTasks();
  }

  async function deleteTask(id) {
    await fetch(`${API}/${id}`, { method: 'DELETE' });
    fetchTasks();
  }
</script>

<main>
  <h1>Task Manager</h1>

  <form onsubmit={addTask}>
    <input bind:value={newTitle} placeholder="Nouvelle tâche..." />
    <select bind:value={newType}>
      <option value="chore">Chore</option>
      <option value="bug">Bug</option>
      <option value="feature">Feature</option>
    </select>
    <button type="submit">Ajouter</button>
  </form>

  <ul>
    {#each tasks as task (task.id)}
      <li class:completed={task.completed}>
        <span>
          <strong>{task.title}</strong> <em>[{task.task_type}]</em>
        </span>
        <button onclick={() => toggleTask(task.id, task.completed)}>
          {task.completed ? '✔' : '⬜'}
        </button>
        <button onclick={() => deleteTask(task.id)}>🗑️</button>
      </li>
    {/each}
  </ul>
</main>

<style>
  .completed span {
    text-decoration: line-through;
    opacity: 0.6;
  }
  li {
    list-style: none;
    margin: 0.5rem 0;
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
</style>
