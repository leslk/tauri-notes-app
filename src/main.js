const { invoke } = window.__TAURI__.tauri;

// Display the notes when Dom is loaded
window.addEventListener("DOMContentLoaded", async () => {
  const loadNotes = new LoadNotes();
  await loadNotes.loadNotes();
});

let isUpdate = false; // Variable to track if it's an update operation
let createdNoteId = null; // Variable to track the note id for delete operation or update operation

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
      // await invoke("save_note", { id: this.id, title: this.title, content: this.content });
      await invoke("db_save_note", {
        id: this.id,
        title: this.title,
        content: this.content,
      });
      console.log("Note saved successfully!");
    } catch (error) {
      console.error("Failed to save note:", error);
    }
  }

  async updateNote() {
    try {
      // await invoke("update_note", {
      //   id: this.id,
      //   title: this.title,
      //   content: this.content,
      // });
      await invoke("db_update_note", {
        id: this.id,
        title: this.title,
        content: this.content,
      });
      console.log("Note updated successfully!");
    } catch (error) {
      console.error("Failed to update note:", error);
    }
  }

  async deleteNote() {
    try {
      // await invoke("delete_note", { id: this.id });
      await invoke("db_delete_note", { id: this.id });
      console.log("Note deleted successfully!");
      const loadNotes = new LoadNotes();
      await loadNotes.loadNotes(); // Reload notes after operation
    } catch (error) {
      console.error("Failed to delete note:", error);
    }
  }

  async saveOrUpdateNote() {
    const content = document.getElementById("note-text").value;
    const title = document.getElementById("note-title").value;
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
  }
}

// Class to load notes
class LoadNotes {
  constructor() {
    this.notes = [];
  }

  async loadNotes() {
    try {
      // this.notes = await invoke("load_notes");
      this.notes = await invoke("db_load_notes");
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
      return;
    }
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

      div.addEventListener("click", () => {
        if (target === trashIcon || target === pencilIcon) return;
        const previewNote = new PreviewNote();
        previewNote.createPreview(this.notes[i]);
      });

      div.appendChild(header);
      const content = document.createElement("div");
      content.classList.add("note-content");
      const title = document.createElement("h2");
      title.textContent = this.notes[i].title;
      const p = document.createElement("p");
      p.textContent = this.notes[i].content;
      content.appendChild(title);
      content.appendChild(p);
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
    noteText.textContent = note.content;
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
    this.textInput = document.getElementById("note-text");
    this.submitBtn = document.getElementById("note-submit");
    this.modalHeaderText = document.getElementById("modal-header-text");
  }

  openModal(title = "", text = "") {
    this.titleInput.value = title;
    this.textInput.value = text;
    isUpdate = createdNoteId ? true : false; // Reset the update variable
    this.submitBtn.textContent = isUpdate ? "Update Note" : "Save Note";
    this.modalHeaderText.textContent = isUpdate
      ? "Update Note"
      : "Add New Note";
    document.getElementById("add-note-modal").style.display = "block";
  }

  closeModal() {
    document.getElementById("add-note-modal").style.display = "none";
    isUpdate = false; // Reset the update variable
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
