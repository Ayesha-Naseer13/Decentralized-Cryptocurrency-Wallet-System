"use client"

import { useState, useEffect } from "react"
import "../styles/Dashboard.css"

const Dashboard = ({ user }) => {
  const [balance, setBalance] = useState(0)
  const [utxos, setUtxos] = useState([])
  const [zakat, setZakat] = useState(0)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetchDashboardData()
  }, [user])

  const fetchDashboardData = async () => {
    try {
      const token = localStorage.getItem("authToken")
      const response = await fetch(`http://localhost:3001/api/wallet/${user.walletId}`, {
        headers: { Authorization: `Bearer ${token}` },
      })

      if (response.ok) {
        const data = await response.json()
        setBalance(data.balance)
        setUtxos(data.utxos)
        setZakat(data.zakatDeduction)
      }
    } catch (err) {
      console.error("Failed to fetch dashboard data", err)
    } finally {
      setLoading(false)
    }
  }

  if (loading) return <div className="loading">Loading...</div>

  return (
    <div className="dashboard">
      <div className="dashboard-container">
        <h1>Wallet Dashboard</h1>

        <div className="dashboard-grid">
          <div className="card balance-card">
            <h3>Total Balance</h3>
            <p className="balance">{balance} CWC</p>
            <small>Wallet ID: {user.walletId}</small>
          </div>

          <div className="card zakat-card">
            <h3>Monthly Zakat (2.5%)</h3>
            <p className="zakat">{zakat} CWC</p>
            <small>Auto-deducted monthly</small>
          </div>

          <div className="card utxo-card">
            <h3>UTXOs</h3>
            <p className="count">{utxos.length}</p>
            <small>Unspent transaction outputs</small>
          </div>
        </div>

        <div className="utxo-list">
          <h2>Your UTXOs</h2>
          {utxos.length > 0 ? (
            <table className="utxo-table">
              <thead>
                <tr>
                  <th>UTXO ID</th>
                  <th>Amount</th>
                  <th>Status</th>
                  <th>Block Hash</th>
                </tr>
              </thead>
              <tbody>
                {utxos.map((utxo) => (
                  <tr key={utxo.id}>
                    <td className="utxo-id">{utxo.id.substring(0, 16)}...</td>
                    <td>{utxo.amount} CWC</td>
                    <td>
                      <span className={`status ${utxo.status}`}>{utxo.status}</span>
                    </td>
                    <td className="block-hash">{utxo.blockHash.substring(0, 16)}...</td>
                  </tr>
                ))}
              </tbody>
            </table>
          ) : (
            <p className="no-data">No UTXOs available</p>
          )}
        </div>
      </div>
    </div>
  )
}

export default Dashboard
