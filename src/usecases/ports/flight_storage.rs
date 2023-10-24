pub trait FlightStorage {
    fn store(flight : Flight) -> boolean;
    fn fetch(flightId : String) -> Fligth;
    fn delete(flightId : String) -> Flight;
}