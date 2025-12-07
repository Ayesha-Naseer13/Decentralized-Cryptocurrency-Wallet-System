"use client"

import { useState } from "react"
import "../styles/SendMoney.css"

const SendMoney = ({ user }) => {
  const [formData, setFormData] = useState({
    recipientWallet: "",
    amount: "",
    note: "",
  })
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState("")
  const [txHash, setTxHash] = useState("")

  const handleChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    })
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    setLoading(true)
    setMessage("")
    setTxHash("")

    try {
      const token = localStorage.getItem("authToken")
      const response = await fetch("http://localhost:3001/api/transactions/send", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          senderWalletId: user.walletId,
          recipientWalletId: formData.recipientWallet,
          amount: Number.parseFloat(formData.amount),
          note: formData.note,
        }),
      })

      if (response.ok) {
        const data = await response.json()
        setTxHash(data.transactionHash)
        setMessage("Transaction created successfully!")
        setFormData({ recipientWallet: "", amount: "", note: "" })
      } else {
        setMessage("Transaction failed")
      }
    } catch (err) {
      setMessage("Error sending money: " + err.message)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="send-money">
      <div className="send-container">
        <h1>Send Money</h1>

        <form onSubmit={handleSubmit} className="send-form">
          <div className="form-group">
            <label>Recipient Wallet ID</label>
            <input
              type="text"
              name="recipientWallet"
              value={formData.recipientWallet}
              onChange={handleChange}
              placeholder="Enter recipient's wallet ID"
              required
            />
          </div>

          <div className="form-group">
            <label>Amount (CWC)</label>
            <input
              type="number"
              name="amount"
              value={formData.amount}
              onChange={handleChange}
              placeholder="0.00"
              step="0.01"
              required
            />
          </div>

          <div className="form-group">
            <label>Message (Optional)</label>
            <textarea
              name="note"
              value={formData.note}
              onChange={handleChange}
              placeholder="Add a note to your transaction"
            />
          </div>

          <button type="submit" disabled={loading} className="submit-btn">
            {loading ? "Processing..." : "Send Money"}
          </button>
        </form>

        {message && (
          <div className="message-box">
            <p>{message}</p>
            {txHash && <p className="tx-hash">TX: {txHash}</p>}
          </div>
        )}
      </div>
    </div>
  )
}

export default SendMoney
