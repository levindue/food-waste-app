import { MapContainer, TileLayer, Marker } from 'react-leaflet';
import '../styles/Map.css';
import 'leaflet/dist/leaflet.css'
import { Icon } from 'leaflet';
import PopupContent from './PopupContent';
import List from '../components/List';
import { useState, useEffect } from 'react';

const myIcon = new Icon({
  iconUrl: "https://unpkg.com/leaflet@1.9.3/dist/images/marker-icon.png",
  iconSize: [25, 41]
})

async function get_data() {
  try {
    const response = await fetch("http://localhost:8080/api/list_people");
    const data = await response.json();
    return data;
  } catch (error) {
    console.log(error);
  }
}

function Map() {
  const position = [51.97509, 7.40953]
  const levin_position = [51.97509, 7.40953]
  const jonas_position = [51.97509, 7.41999]
  const marvin_position = [51.97809, 7.40992]

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
    <div className="Map">

      <MapContainer center={position} zoom={15} scrollWheelZoom={false}>
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        <Marker position={levin_position} icon={myIcon}>
          <PopupContent name={"Alice"} food={["Pizza"]} />
        </Marker>

        <Marker position={jonas_position} icon={myIcon}>
          <PopupContent name={"Charlie"} food={["Pasta", "Salat"]} />
        </Marker>
      </MapContainer>

      <div className="ListContainer">
        <List />
      </div>
    </div>
  )
}

export default Map;
