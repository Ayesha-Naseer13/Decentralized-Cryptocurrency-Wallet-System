"use client"

import { useState, useEffect } from "react"
import "../styles/History.css"

const TransactionHistory = ({ user }) => {
  const [transactions, setTransactions] = useState([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetchTransactions()
  }, [user])

  const fetchTransactions = async () => {
    try {
      const token = localStorage.getItem("authToken")
      const response = await fetch(`http://localhost:3001/api/transactions/history/${user.walletId}`, {
        headers: { Authorization: `Bearer ${token}` },
      })

      if (response.ok) {
        const data = await response.json()
        setTransactions(data)
      }
    } catch (err) {
      console.error("Failed to fetch transactions", err)
    } finally {
      setLoading(false)
    }
  }

  if (loading) return <div className="loading">Loading...</div>

  return (
    <div className="history">
      <div className="history-container">
        <h1>Transaction History</h1>

        {transactions.length > 0 ? (
          <table className="history-table">
            <thead>
              <tr>
                <th>Type</th>
                <th>Amount</th>
                <th>Counterparty</th>
                <th>Timestamp</th>
                <th>Status</th>
                <th>Block Hash</th>
              </tr>
            </thead>
            <tbody>
              {transactions.map((tx) => (
                <tr key={tx.id}>
                  <td>
                    <span className={`type ${tx.type}`}>{tx.type}</span>
                  </td>
                  <td>{tx.amount} CWC</td>
                  <td>{tx.counterparty}</td>
                  <td>{new Date(tx.timestamp).toLocaleString()}</td>
                  <td>
                    <span className={`status ${tx.status}`}>{tx.status}</span>
                  </td>
                  <td className="block-hash">{tx.blockHash?.substring(0, 16)}...</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <p className="no-data">No transactions yet</p>
        )}
      </div>
    </div>
  )
}

export default TransactionHistory
