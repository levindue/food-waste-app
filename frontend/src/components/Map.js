import { MapContainer, TileLayer, Marker } from 'react-leaflet';
import './Map.css';
import 'leaflet/dist/leaflet.css'
import { Icon } from 'leaflet';
import PopupContent from './PopupContent';

const myIcon = new Icon({
  iconUrl: "https://unpkg.com/leaflet@1.9.3/dist/images/marker-icon.png",
  iconSize: [25, 41]
})

function Map() {
  const position = [51.97509, 7.40953]

  return (
    <div className="Map">
      <MapContainer center={position} zoom={15} scrollWheelZoom={false}>
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        <Marker position={position} icon={myIcon}>
          <PopupContent name={"Levin Düsterhus"} food={["Brot", "3 Äpfel", "5 Bananen"]} />
        </Marker>
      </MapContainer>
    </div>
  )
}

export default Map;
