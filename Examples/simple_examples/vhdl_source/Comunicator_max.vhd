
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.numeric_std .ALL;


entity Comunicator is
Generic(
    -- the ammount of bit to recive
    RECIVING_SIZE: integer := 96;

    -- the ammount of bit to recive
    SENDING_SIZE: integer := 32;

    -- the numer of clock cicle the macine wait before sending the data back
    CLOCK_REPLYING_TIMER: integer := 1000

);
Port(
    CLK: in std_logic;
    TX: out std_logic;
    RX: in std_logic;
    reset: in std_logic;
    waiting_indicator: out std_logic;
    loading_indicator: out std_logic;
    weating_sending_indicator: out std_logic
);
end Comunicator;

architecture Behavioral of Comunicator is

    Type StateType is (Waiting,Loading,Weating_sending,Weating_sending2,Weating_sending3);

    signal state: StateType := Waiting;

    signal TX_reciver: std_logic := '1';
    signal RX_reciver: std_logic := '1';

    signal TX_transmeater: std_logic := '1';
    signal RX_transmeater: std_logic := '1';


    signal data_ready_reciver:  std_logic := '0';

    signal data_in: std_logic_vector(RECIVING_SIZE-1 downto 0);
    signal data_out: std_logic_vector(SENDING_SIZE-1 downto 0);

    signal transmeater_trigger: std_logic := '0';
    signal transmeater_finishd: std_logic := '0';

    signal counter: unsigned(31 downto 0);

    signal reset_signal:  std_logic := '1';

    signal second_reset:  std_logic := '1';

    -- inputs
    signal input_n1: signed(31 downto 0);
signal input_n2: signed(31 downto 0);
signal input_n3: signed(31 downto 0);


    --output
    signal output: signed(31 downto 0);


begin

    transmeater: entity work.SafeTransmeater
        Generic map(
            RECIVING_SIZE =>  SENDING_SIZE
        )
        port map(
          CLK => CLK,
          TX =>TX_transmeater,
          RX =>RX_transmeater,
          reset =>reset_signal,
          input_trigger => transmeater_trigger,
          input =>data_out,
          send_finished => transmeater_finishd
        );

    reciver: entity work.SafeReciver
        Generic map(
            RECIVING_SIZE =>  RECIVING_SIZE
        )
        port map(
            CLK => CLK,
            RX => RX_reciver,
            TX => TX_reciver,
            reset => reset_signal,
            output_ready => data_ready_reciver,
            output => data_in
        );


    --TX <= TX_reciver when state = Waiting else TX_transmeater;
    TX <= TX_reciver and TX_transmeater;


    RX_reciver <= RX when state = Waiting else '1';
    RX_transmeater <= '1' when state = Waiting else RX;

    waiting_indicator <= '1' when state = Waiting else '0';
    loading_indicator <= '1' when state = Loading else '0';
    weating_sending_indicator <= '1' when state = Weating_sending else '0';

    reset_signal <= reset and second_reset;

    main: process(CLK,reset) begin


        if reset = '0' then
            state <= Waiting;
        end if;

        if rising_edge(CLK) then

            if state = Waiting then
                second_reset <= '1';
                if data_ready_reciver = '1' then
                    state <= Loading;
                    counter <= TO_UNSIGNED(0,32);
                end if;

            elsif state = Loading then
                counter <= counter+1;

                if counter >  TO_UNSIGNED(CLOCK_REPLYING_TIMER,32)then
                    transmeater_trigger <= '1';
                end if;

                if counter >  TO_UNSIGNED(CLOCK_REPLYING_TIMER+2,32)then
                    transmeater_trigger <= '0';
                end if;

                if counter >  TO_UNSIGNED(CLOCK_REPLYING_TIMER+4,32)then
                    state <= Weating_sending;
                end if;


            elsif state = Weating_sending then
                if transmeater_finishd = '1' then
                    state <= Weating_sending2;
                    second_reset <= '0';
                end if;

            elsif state = Weating_sending2 then
                    state <= Weating_sending3;
            elsif state = Weating_sending3 then
                    state <= Waiting;
            end if;


        end if;

    end process;

    --constructing inputs
    input_n1 <= signed(data_in(31 downto 0));
input_n2 <= signed(data_in(63 downto 32));
input_n3 <= signed(data_in(95 downto 64));


    --deconstruction outputs
    data_out(31 downto 0) <= std_logic_vector(output);

    
    --      INSERT HERE YOUR VHDL CODE
    output <= input_n1 when input_n1 >= input_n2 and input_n1 >= input_n3 else input_n2 when input_n2 >= input_n3 else input_n3;


end Behavioral;


