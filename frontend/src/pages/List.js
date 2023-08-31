import React, { useState, useEffect } from 'react';
import Header from '../components/Header';
import '../styles/List.css'

async function get_data() {
  try {
    const response = await fetch("http://localhost:8080/api/list_people");
    const data = await response.json();
    return data;
  } catch (error) {
    console.log(error);
  }
}

function Test() {
  const [data, setData] = useState([]);

  useEffect(() => {
    async function fetchData() {
      const fetchedData = await get_data();
      setData(fetchedData);
    }
    fetchData();
  }, []);

  // filter if no food
  const filteredData = data.filter(person => person.food.length > 0);

  return (
    <div className="container">
    <Header />
      <h1>Liste</h1>
      <ul>
        {filteredData.map(person => (
          <li key={person.id}>
            <strong>Name:</strong> {person.name} <br />
            <ul className="food-list">
              {person.food.map(foodItem => (
                <li key={foodItem.id}>{foodItem.name}</li>
              ))}
            </ul>
          </li>
        ))}
      </ul>
    </div>
  );
}


export default Test;
