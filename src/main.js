const { invoke } = window.__TAURI__.tauri;

// Display the notes when Dom is loaded
window.addEventListener("DOMContentLoaded", async () => {
  await loadNotes();
});

let isUpdate = false; // Variable to track if it's an update operation
let createdNoteId = null; // Variable to track the note id for delete operation or update operation

// Function to open the modal for creating or updating a note
function openModal(title = "", text = "") {
  const titleInput = document.getElementById("note-title");
  const textInput = document.getElementById("note-text");
  const submitBtn = document.getElementById("note-submit");
  const modalHeaderText = document.getElementById("modal-header-text");

  // Set initial values
  titleInput.value = title;
  textInput.value = text;
  isUpdate = createdNoteId ? true : false; // Reset the update variable

  // Change the submit button text based on operation
  submitBtn.textContent = isUpdate ? "Update Note" : "Save Note";
  modalHeaderText.textContent = isUpdate ? "Update Note" : "Add New Note";

  // Show the modal
  document.getElementById("add-note-modal").style.display = "block";
}

// Function to close the modal
function closeModal() {
  document.getElementById("add-note-modal").style.display = "none";
  isUpdate = false; // Reset the update variable
}

// Function that defines whether it's an update or save operation
async function saveOrUpdateNote() {
  const content = document.getElementById("note-text").value;
  const title = document.getElementById("note-title").value;
  if (isUpdate) {
    await updateNote(content, title); // Call update function if it's an update operation
  } else {
    await saveNote(content, title); // Call save function if it's a save operation
  }
  closeModal(); // Close the modal after operation
}

// When the user clicks anywhere outside of the modal, close it
window.onclick = function (event) {
  const addNoteModal = document.getElementById("add-note-modal");
  if (event.target == addNoteModal) {
    addNoteModal.style.display = "none";
  }
};

async function saveNote(text, title) {
  try {
    const noteId = Math.floor(Math.random() * 1000000);
    await invoke("save_note", { id: noteId, title: title, content: text });
    console.log("Note saved successfully!");
  } catch (error) {
    console.error("Failed to save note:", error);
  }
}

async function updateNote(text, title) {
  try {
    await invoke("update_note", {
      id: createdNoteId,
      title: title,
      content: text,
    });
    console.log("Note updated successfully!");
  } catch (error) {
    console.error("Failed to update note:", error);
  }
}

async function deleteNote(id) {
  try {
    await invoke("delete_note", { id: id });
    console.log("Note deleted successfully!");
    await loadNotes(); // Reload notes after operation
  } catch (error) {
    console.error("Failed to delete note:", error);
  }
}

function createPreview(note) {
  const noteContent = document.getElementById("note-preview");
  noteContent.innerHTML = ""; // Clear previous note
  const noteTitle = document.createElement("h2");
  noteTitle.textContent = note.title;
  const noteText = document.createElement("p");
  noteText.textContent = note.content;
  noteContent.appendChild(noteTitle);
  noteContent.appendChild(noteText);
}

async function loadNotes() {
  try {
    const notes = await invoke("load_notes");
    const noteContainer = document.getElementById("notes");
    noteContainer.innerHTML = ""; // Clear previous notes
    for (let i = 0; i < notes.length; i++) {
      if (i === 0) {
        createPreview(notes[i]);
      }
      const div = document.createElement("div");
      div.classList.add("note-card");
      div.setAttribute("data-id", notes[i].id);
      const header = document.createElement("div");
      header.classList.add("note-header");
      const pencilIcon = document.createElement("i");
      pencilIcon.classList.add("fas", "fa-pencil-alt");
      const trashIcon = document.createElement("i");
      trashIcon.classList.add("fas", "fa-trash-alt");
      header.appendChild(pencilIcon);
      header.appendChild(trashIcon);

      pencilIcon.addEventListener("click", () => {
        createdNoteId = notes[i].id;
        openModal(notes[i].title, notes[i].content);
      });

      trashIcon.addEventListener("click", () => {
        deleteNote(notes[i].id);
      });

      div.addEventListener("click", () => {
        createPreview(notes[i]);
      });

      div.appendChild(header);
      const content = document.createElement("div");
      content.classList.add("note-content");
      const title = document.createElement("h2");
      title.textContent = notes[i].title;
      const p = document.createElement("p");
      p.textContent = notes[i].content;
      content.appendChild(title);
      content.appendChild(p);
      div.appendChild(content);
      noteContainer.appendChild(div);
      console.log("Notes loaded successfully!");
    }
  } catch (error) {
    console.error("Failed to load notes:", error);
  }
}

document
  .getElementById("add-note")
  .addEventListener("click", () => openModal());
document
  .getElementById("note-close")
  .addEventListener("click", () => closeModal());
document.getElementById("note-submit").addEventListener("click", (e) => {
  e.preventDefault();
  saveOrUpdateNote();
});
