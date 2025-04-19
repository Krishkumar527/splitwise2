import React, { useState } from "react";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../declarations/splitwise2_backend";

const agent = new HttpAgent();
const backend = Actor.createActor(idlFactory, {
    agent,
    canisterId: "your-backend-canister-id", // Replace with your actual backend canister ID
});

const Home = () => {
    const [payer, setPayer] = useState("");
    const [amount, setAmount] = useState(0);
    const [participants, setParticipants] = useState("");
    const [balances, setBalances] = useState([]);

    const handleAddExpense = async () => {
        const participantsArray = participants.split(",").map((p) => p.trim());
        try {
            await backend.add_expense(payer, parseInt(amount), participantsArray);
            alert("Expense added successfully!");
        } catch (error) {
            console.error("Error adding expense:", error);
            alert("Failed to add expense.");
        }
    };

    const handleGetBalances = async () => {
        try {
            const result = await backend.get_all_user_balances();
            setBalances(result);
        } catch (error) {
            console.error("Error fetching balances:", error);
            alert("Failed to fetch balances.");
        }
    };

    return (
        <div>
            <h1>Splitwise DApp</h1>
            <div>
                <input
                    type="text"
                    placeholder="Payer Principal"
                    value={payer}
                    onChange={(e) => setPayer(e.target.value)}
                />
                <input
                    type="number"
                    placeholder="Amount"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                />
                <input
                    type="text"
                    placeholder="Participants (comma-separated Principals)"
                    value={participants}
                    onChange={(e) => setParticipants(e.target.value)}
                />
                <button onClick={handleAddExpense}>Add Expense</button>
            </div>
            <div>
                <button onClick={handleGetBalances}>Get All Balances</button>
                <ul>
                    {balances.map(([principal, balance], index) => (
                        <li key={index}>
                            {principal}: {balance}
                        </li>
                    ))}
                </ul>
            </div>
        </div>
    );
};

export default Home;