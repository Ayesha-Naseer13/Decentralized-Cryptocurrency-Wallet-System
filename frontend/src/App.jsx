"use client"

import React, { useState, useEffect } from "react"
import "./styles/App.css"
import Navbar from "./components/Navbar"
import Dashboard from "./pages/Dashboard"
import Login from "./pages/Login"
import Register from "./pages/Register"
import SendMoney from "./pages/SendMoney"
import BlockExplorer from "./pages/BlockExplorer"
import TransactionHistory from "./pages/TransactionHistory"


const App = () => {
  const [isLoggedIn, setIsLoggedIn] = useState(false)
  const [currentPage, setCurrentPage] = useState("dashboard")
  const [user, setUser] = useState(null)

  useEffect(() => {
    const token = localStorage.getItem("authToken")
    const userData = localStorage.getItem("userData")
    if (token && userData) {
      setIsLoggedIn(true)
      setUser(JSON.parse(userData))
    }
  }, [])

  const handleLogin = (userData) => {
    setUser(userData)
    setIsLoggedIn(true)
    setCurrentPage("dashboard")
  }

  const handleLogout = () => {
    localStorage.removeItem("authToken")
    localStorage.removeItem("userData")
    setIsLoggedIn(false)
    setUser(null)
    setCurrentPage("login")
  }

  if (!isLoggedIn) {
    return currentPage === "login" ? (
      <Login onLogin={handleLogin} onSwitchToRegister={() => setCurrentPage("register")} />
    ) : (
      <Register onRegister={handleLogin} onSwitchToLogin={() => setCurrentPage("login")} />
    )
  }

  return (
    <div className="app">
      <Navbar user={user} onLogout={handleLogout} onNavigate={setCurrentPage} />
      <main className="main-content">
        {currentPage === "dashboard" && <Dashboard user={user} />}
        {currentPage === "send" && <SendMoney user={user} />}
        {currentPage === "history" && <TransactionHistory user={user} />}
        {currentPage === "explorer" && <BlockExplorer />}
      </main>
    </div>
  )
}

export default App
