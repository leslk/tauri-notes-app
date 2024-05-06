const { invoke } = window.__TAURI__.tauri;

// Display the notes when Dom is loaded
window.addEventListener("DOMContentLoaded", async () => {
  changeTypeOfSaving(typeOfSaving); // Set the type of saving
  const loadNotes = new LoadNotes();
  await loadNotes.loadNotes();
});
let isUpdate = false; // Variable to track if it's an update operation
let createdNoteId = null; // Variable to track the note id for delete operation or update operation
let typeOfSaving = "json"; // Variable to track the type of saving
let queryText = ""; // Variable to track the search query

function changeTypeOfSaving(type) {
  const jsonSaving = document.getElementById("json");
  const databaseSaving = document.getElementById("database");
  typeOfSaving = type;
  if (type === "json") {
    const jsonSaving = document.getElementById("json");
    jsonSaving.classList.add("active");
    databaseSaving.classList.remove("active");
  } else {
    databaseSaving.classList.add("active");
    jsonSaving.classList.remove("active");
  }
}

// When the user clicks anywhere outside of the modal, close it
window.onclick = function (event) {
  const modal = new Modal();
  if (event.target === document.getElementById("add-note-modal")) {
    modal.closeModal();
  }
};

// class to handle notes
class Notes {
  constructor(id, title, content) {
    this.id = id;
    this.title = title;
    this.content = content;
  }

  async saveNote() {
    try {
      if (typeOfSaving === "json") {
        await invoke("save_note", {
          id: this.id,
          title: this.title,
          content: this.content,
        });
      } else {
        await invoke("db_save_note", {
          id: this.id,
          title: this.title,
          content: this.content,
        });
      }
      console.log("Note saved successfully!");
    } catch (error) {
      console.error("Failed to save note:", error);
    }
  }

  async updateNote() {
    try {
      if (typeOfSaving === "json") {
        await invoke("update_note", {
          id: this.id,
          title: this.title,
          content: this.content,
        });
      } else {
        await invoke("db_update_note", {
          id: this.id,
          title: this.title,
          content: this.content,
        });
      }
      console.log("Note updated successfully!");
    } catch (error) {
      console.error("Failed to update note:", error);
    }
  }

  async deleteNote() {
    try {
      if (typeOfSaving === "json") {
        await invoke("delete_note", { id: this.id });
      } else {
        await invoke("db_delete_note", { id: this.id });
      }
      console.log("Note deleted successfully!");
      const loadNotes = new LoadNotes();
      await loadNotes.loadNotes(); // Reload notes after operation
    } catch (error) {
      console.error("Failed to delete note:", error);
    }
  }

  async saveOrUpdateNote() {
    const quill = new Quill("#editor");
    const content = quill.root.innerHTML;
    const title = document.getElementById("note-title").value;
    const error = document.querySelector(".error");
    if (error) {
      error.remove();
    }
    if (!title || content === "<p><br></p>") {
      const error = document.createElement("p");
      error.classList.add("error");
      const modalForm = document.getElementById("modal-form");
      error.textContent = "Title and content cannot be empty!";
      modalForm.appendChild(error);
      return;
    }
    if (isUpdate) {
      const note = new Notes(createdNoteId, title, content);
      await note.updateNote(content, title); // Call update function if it's an update operation
    } else {
      const noteId = Math.floor(Math.random() * 1000000);
      const note = new Notes(noteId, title, content);
      await note.saveNote(content, title); // Call save function if it's a save operation
    }
    const loadNotes = new LoadNotes();
    await loadNotes.loadNotes(); // Reload notes after operation
    const modal = new Modal();
    modal.closeModal();
    createdNoteId = null; // Reset the note id
    isUpdate = false; // Reset the update variable
  }
}

// Class to load notes
class LoadNotes {
  constructor() {
    this.notes = [];
  }

  async loadNotes() {
    try {
      if (typeOfSaving === "json") {
        this.notes = await invoke("load_notes");
      } else {
        this.notes = await invoke("db_load_notes", { querySearch: queryText });
      }
      const displayNotes = new DisplayNotes(this.notes);
      displayNotes.displayNotes();
    } catch (error) {
      console.error("Failed to load notes:");
    }
  }
}

// Class to display notes
class DisplayNotes {
  constructor(notes) {
    this.notes = notes;
  }

  displayNotes() {
    const noteContainer = document.getElementById("notes");
    noteContainer.innerHTML = ""; // Clear previous notes
    if (this.notes.length === 0) {
      const previewNote = new PreviewNote();
      previewNote.createNoContentPreview();
      document.getElementById("notes").style.display = "none";
      return;
    }
    document.getElementById("notes").style.display = "flex";
    for (let i = 0; i < this.notes.length; i++) {
      if (i === 0) {
        const previewNote = new PreviewNote();
        previewNote.createPreview(this.notes[i]);
      }
      const div = document.createElement("div");
      div.classList.add("note-card");
      div.setAttribute("data-id", this.notes[i].id);
      const header = document.createElement("div");
      header.classList.add("note-header");
      const pencilIcon = document.createElement("i");
      pencilIcon.classList.add("fas", "fa-pencil-alt");
      const trashIcon = document.createElement("i");
      trashIcon.classList.add("fas", "fa-trash-alt");
      header.appendChild(pencilIcon);
      header.appendChild(trashIcon);

      pencilIcon.addEventListener("click", () => {
        createdNoteId = this.notes[i].id;
        const modal = new Modal();
        isUpdate = true;
        modal.openModal(this.notes[i].title, this.notes[i].content);
      });

      trashIcon.addEventListener("click", async () => {
        const note = new Notes(
          this.notes[i].id,
          this.notes[i].title,
          this.notes[i].content
        );
        await note.deleteNote(this.notes[i].id);
      });

      div.addEventListener("click", (event) => {
        if (event.target === trashIcon || event.target === pencilIcon) return;
        const previewNote = new PreviewNote();
        previewNote.createPreview(this.notes[i]);
      });

      div.appendChild(header);
      const content = document.createElement("div");
      content.classList.add("note-content");
      const titleContainer = document.createElement("div");
      titleContainer.classList.add("note-title");
      const titleLabel = document.createElement("h2");
      titleLabel.textContent = "Title: ";
      const title = document.createElement("h2");
      title.classList.add("note-title-text");
      title.textContent = this.notes[i].title || "No title";
      titleContainer.appendChild(titleLabel);
      titleContainer.appendChild(title);
      const contentContainer = document.createElement("div");
      contentContainer.classList.add("note-content-container");
      const contentLabel = document.createElement("h2");
      contentLabel.textContent = "Content: ";
      const p = document.createElement("p");
      if (this.notes[i].content == "<p><br></p>") {
        p.textContent = "No content";
      }
      p.innerHTML = this.notes[i].content;
      contentContainer.appendChild(contentLabel);
      contentContainer.appendChild(p);
      content.appendChild(titleContainer);
      content.appendChild(contentContainer);
      div.appendChild(content);
      noteContainer.appendChild(div);
    }
  }
}

// Class to preview note
class PreviewNote {
  constructor() {
    this.noteContent = document.getElementById("note-preview");
  }

  createPreview(note) {
    this.noteContent.innerHTML = ""; // Clear previous note
    const noteTitle = document.createElement("h2");
    noteTitle.textContent = note.title;
    const noteText = document.createElement("p");
    noteText.innerHTML = note.content;
    this.noteContent.appendChild(noteTitle);
    this.noteContent.appendChild(noteText);
  }

  createNoContentPreview() {
    this.noteContent.innerHTML = "";
    const noContentTitle = document.createElement("h2");
    noContentTitle.textContent = "No notes found!";
    this.noteContent.appendChild(noContentTitle);
  }
}

// class to handle Modal
class Modal {
  constructor() {
    this.titleInput = document.getElementById("note-title");
    this.submitBtn = document.getElementById("note-submit");
    this.modalHeaderText = document.getElementById("modal-header-text");
    this.quill = null;
  }

  openModal(title = "", text = "") {
    this.titleInput.value = title;
    this.submitBtn.textContent = isUpdate ? "Update Note" : "Save Note";
    this.modalHeaderText.textContent = isUpdate
      ? "Update Note"
      : "Add New Note";
    document.getElementById("add-note-modal").style.display = "flex";
    const editor = document.createElement("div");
    editor.id = "editor";
    document.getElementById("modal-form").appendChild(editor);
    this.quill = new Quill("#editor", {
      theme: "snow",
    });
    this.quill.root.innerHTML = text;
  }

  closeModal() {
    document.getElementById("add-note-modal").style.display = "none";
    document.querySelector(".ql-toolbar.ql-snow").remove();
  }
}

// Event listeners

// Event listener to open modal
document.getElementById("add-note").addEventListener("click", () => {
  const modal = new Modal();
  modal.openModal();
});

// Event listener to close modal
document.getElementById("note-close").addEventListener("click", () => {
  const modal = new Modal();
  modal.closeModal();
});

// Event listener to save or update note
document.getElementById("note-submit").addEventListener("click", async (e) => {
  e.preventDefault();
  const note = new Notes();
  await note.saveOrUpdateNote();
});

// Event listener to export notes
document.getElementById("export-button").addEventListener("click", async () => {
  try {
    if (typeOfSaving === "json") {
      await invoke("export_notes_to_json");
    } else {
      await invoke("db_export_notes_to_json");
    }
  } catch (error) {
    console.error("Failed to export notes:", error);
  }
});

// Event listener to change type of saving
document.getElementById("json").addEventListener("click", async () => {
  changeTypeOfSaving("json");
  const loadNotes = new LoadNotes();
  await loadNotes.loadNotes();
});

document.getElementById("database").addEventListener("click", async () => {
  changeTypeOfSaving("database");
  const loadNotes = new LoadNotes();
  await loadNotes.loadNotes();
});

// Event listener to search notes
document.getElementById("search-notes").addEventListener("input", async (e) => {
  queryText = e.target.value;
  console.log(queryText);
  const loadNotes = new LoadNotes();
  await loadNotes.loadNotes();
});
