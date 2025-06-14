package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"strconv"
	"sync"
)

type Job struct {
	ID   int    `json:"id"`
	Data string `json:"data"`
}

type Result struct {
	ID        int    `json:"id"`
	Processed string `json:"processed"`
}

var (
	jobChan   chan Job
	resultMap sync.Map
	idCounter int
)

func worker() {
	for job := range jobChan {
		processed := fmt.Sprintf("processed: %s", job.Data)
		resultMap.Store(job.ID, processed)
	}
}

func enqueueHandler(w http.ResponseWriter, r *http.Request) {
	var payload struct{ Data string }
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "bad payload", http.StatusBadRequest)
		return
	}
	idCounter++
	job := Job{ID: idCounter, Data: payload.Data}
	jobChan <- job
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(job)
}

func resultHandler(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	id, err := strconv.Atoi(idStr)
	if err != nil {
		http.Error(w, "bad id", http.StatusBadRequest)
		return
	}
	if value, ok := resultMap.Load(id); ok {
		res := Result{ID: id, Processed: value.(string)}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(res)
	} else {
		http.Error(w, "not ready", http.StatusAccepted)
	}
}

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "9000"
	}
	workers := 4
	jobChan = make(chan Job, 100)
	for i := 0; i < workers; i++ {
		go worker()
	}

	http.HandleFunc("/enqueue", enqueueHandler)
	http.HandleFunc("/result", resultHandler)

	log.Printf("Job queue listening on :%s", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}
