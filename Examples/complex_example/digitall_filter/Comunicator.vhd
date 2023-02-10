
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.numeric_std .ALL;


entity Comunicator is
Generic(
    -- the ammount of bit to recive
    RECIVING_SIZE: integer := 1536;

    -- the ammount of bit to recive
    SENDING_SIZE: integer := 512;

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
    signal input_X0_Y0_r: unsigned(7 downto 0);
signal input_X0_Y0_g: unsigned(7 downto 0);
signal input_X0_Y0_b: unsigned(7 downto 0);
signal input_X0_Y1_r: unsigned(7 downto 0);
signal input_X0_Y1_g: unsigned(7 downto 0);
signal input_X0_Y1_b: unsigned(7 downto 0);
signal input_X0_Y2_r: unsigned(7 downto 0);
signal input_X0_Y2_g: unsigned(7 downto 0);
signal input_X0_Y2_b: unsigned(7 downto 0);
signal input_X0_Y3_r: unsigned(7 downto 0);
signal input_X0_Y3_g: unsigned(7 downto 0);
signal input_X0_Y3_b: unsigned(7 downto 0);
signal input_X0_Y4_r: unsigned(7 downto 0);
signal input_X0_Y4_g: unsigned(7 downto 0);
signal input_X0_Y4_b: unsigned(7 downto 0);
signal input_X0_Y5_r: unsigned(7 downto 0);
signal input_X0_Y5_g: unsigned(7 downto 0);
signal input_X0_Y5_b: unsigned(7 downto 0);
signal input_X0_Y6_r: unsigned(7 downto 0);
signal input_X0_Y6_g: unsigned(7 downto 0);
signal input_X0_Y6_b: unsigned(7 downto 0);
signal input_X0_Y7_r: unsigned(7 downto 0);
signal input_X0_Y7_g: unsigned(7 downto 0);
signal input_X0_Y7_b: unsigned(7 downto 0);
signal input_X1_Y0_r: unsigned(7 downto 0);
signal input_X1_Y0_g: unsigned(7 downto 0);
signal input_X1_Y0_b: unsigned(7 downto 0);
signal input_X1_Y1_r: unsigned(7 downto 0);
signal input_X1_Y1_g: unsigned(7 downto 0);
signal input_X1_Y1_b: unsigned(7 downto 0);
signal input_X1_Y2_r: unsigned(7 downto 0);
signal input_X1_Y2_g: unsigned(7 downto 0);
signal input_X1_Y2_b: unsigned(7 downto 0);
signal input_X1_Y3_r: unsigned(7 downto 0);
signal input_X1_Y3_g: unsigned(7 downto 0);
signal input_X1_Y3_b: unsigned(7 downto 0);
signal input_X1_Y4_r: unsigned(7 downto 0);
signal input_X1_Y4_g: unsigned(7 downto 0);
signal input_X1_Y4_b: unsigned(7 downto 0);
signal input_X1_Y5_r: unsigned(7 downto 0);
signal input_X1_Y5_g: unsigned(7 downto 0);
signal input_X1_Y5_b: unsigned(7 downto 0);
signal input_X1_Y6_r: unsigned(7 downto 0);
signal input_X1_Y6_g: unsigned(7 downto 0);
signal input_X1_Y6_b: unsigned(7 downto 0);
signal input_X1_Y7_r: unsigned(7 downto 0);
signal input_X1_Y7_g: unsigned(7 downto 0);
signal input_X1_Y7_b: unsigned(7 downto 0);
signal input_X2_Y0_r: unsigned(7 downto 0);
signal input_X2_Y0_g: unsigned(7 downto 0);
signal input_X2_Y0_b: unsigned(7 downto 0);
signal input_X2_Y1_r: unsigned(7 downto 0);
signal input_X2_Y1_g: unsigned(7 downto 0);
signal input_X2_Y1_b: unsigned(7 downto 0);
signal input_X2_Y2_r: unsigned(7 downto 0);
signal input_X2_Y2_g: unsigned(7 downto 0);
signal input_X2_Y2_b: unsigned(7 downto 0);
signal input_X2_Y3_r: unsigned(7 downto 0);
signal input_X2_Y3_g: unsigned(7 downto 0);
signal input_X2_Y3_b: unsigned(7 downto 0);
signal input_X2_Y4_r: unsigned(7 downto 0);
signal input_X2_Y4_g: unsigned(7 downto 0);
signal input_X2_Y4_b: unsigned(7 downto 0);
signal input_X2_Y5_r: unsigned(7 downto 0);
signal input_X2_Y5_g: unsigned(7 downto 0);
signal input_X2_Y5_b: unsigned(7 downto 0);
signal input_X2_Y6_r: unsigned(7 downto 0);
signal input_X2_Y6_g: unsigned(7 downto 0);
signal input_X2_Y6_b: unsigned(7 downto 0);
signal input_X2_Y7_r: unsigned(7 downto 0);
signal input_X2_Y7_g: unsigned(7 downto 0);
signal input_X2_Y7_b: unsigned(7 downto 0);
signal input_X3_Y0_r: unsigned(7 downto 0);
signal input_X3_Y0_g: unsigned(7 downto 0);
signal input_X3_Y0_b: unsigned(7 downto 0);
signal input_X3_Y1_r: unsigned(7 downto 0);
signal input_X3_Y1_g: unsigned(7 downto 0);
signal input_X3_Y1_b: unsigned(7 downto 0);
signal input_X3_Y2_r: unsigned(7 downto 0);
signal input_X3_Y2_g: unsigned(7 downto 0);
signal input_X3_Y2_b: unsigned(7 downto 0);
signal input_X3_Y3_r: unsigned(7 downto 0);
signal input_X3_Y3_g: unsigned(7 downto 0);
signal input_X3_Y3_b: unsigned(7 downto 0);
signal input_X3_Y4_r: unsigned(7 downto 0);
signal input_X3_Y4_g: unsigned(7 downto 0);
signal input_X3_Y4_b: unsigned(7 downto 0);
signal input_X3_Y5_r: unsigned(7 downto 0);
signal input_X3_Y5_g: unsigned(7 downto 0);
signal input_X3_Y5_b: unsigned(7 downto 0);
signal input_X3_Y6_r: unsigned(7 downto 0);
signal input_X3_Y6_g: unsigned(7 downto 0);
signal input_X3_Y6_b: unsigned(7 downto 0);
signal input_X3_Y7_r: unsigned(7 downto 0);
signal input_X3_Y7_g: unsigned(7 downto 0);
signal input_X3_Y7_b: unsigned(7 downto 0);
signal input_X4_Y0_r: unsigned(7 downto 0);
signal input_X4_Y0_g: unsigned(7 downto 0);
signal input_X4_Y0_b: unsigned(7 downto 0);
signal input_X4_Y1_r: unsigned(7 downto 0);
signal input_X4_Y1_g: unsigned(7 downto 0);
signal input_X4_Y1_b: unsigned(7 downto 0);
signal input_X4_Y2_r: unsigned(7 downto 0);
signal input_X4_Y2_g: unsigned(7 downto 0);
signal input_X4_Y2_b: unsigned(7 downto 0);
signal input_X4_Y3_r: unsigned(7 downto 0);
signal input_X4_Y3_g: unsigned(7 downto 0);
signal input_X4_Y3_b: unsigned(7 downto 0);
signal input_X4_Y4_r: unsigned(7 downto 0);
signal input_X4_Y4_g: unsigned(7 downto 0);
signal input_X4_Y4_b: unsigned(7 downto 0);
signal input_X4_Y5_r: unsigned(7 downto 0);
signal input_X4_Y5_g: unsigned(7 downto 0);
signal input_X4_Y5_b: unsigned(7 downto 0);
signal input_X4_Y6_r: unsigned(7 downto 0);
signal input_X4_Y6_g: unsigned(7 downto 0);
signal input_X4_Y6_b: unsigned(7 downto 0);
signal input_X4_Y7_r: unsigned(7 downto 0);
signal input_X4_Y7_g: unsigned(7 downto 0);
signal input_X4_Y7_b: unsigned(7 downto 0);
signal input_X5_Y0_r: unsigned(7 downto 0);
signal input_X5_Y0_g: unsigned(7 downto 0);
signal input_X5_Y0_b: unsigned(7 downto 0);
signal input_X5_Y1_r: unsigned(7 downto 0);
signal input_X5_Y1_g: unsigned(7 downto 0);
signal input_X5_Y1_b: unsigned(7 downto 0);
signal input_X5_Y2_r: unsigned(7 downto 0);
signal input_X5_Y2_g: unsigned(7 downto 0);
signal input_X5_Y2_b: unsigned(7 downto 0);
signal input_X5_Y3_r: unsigned(7 downto 0);
signal input_X5_Y3_g: unsigned(7 downto 0);
signal input_X5_Y3_b: unsigned(7 downto 0);
signal input_X5_Y4_r: unsigned(7 downto 0);
signal input_X5_Y4_g: unsigned(7 downto 0);
signal input_X5_Y4_b: unsigned(7 downto 0);
signal input_X5_Y5_r: unsigned(7 downto 0);
signal input_X5_Y5_g: unsigned(7 downto 0);
signal input_X5_Y5_b: unsigned(7 downto 0);
signal input_X5_Y6_r: unsigned(7 downto 0);
signal input_X5_Y6_g: unsigned(7 downto 0);
signal input_X5_Y6_b: unsigned(7 downto 0);
signal input_X5_Y7_r: unsigned(7 downto 0);
signal input_X5_Y7_g: unsigned(7 downto 0);
signal input_X5_Y7_b: unsigned(7 downto 0);
signal input_X6_Y0_r: unsigned(7 downto 0);
signal input_X6_Y0_g: unsigned(7 downto 0);
signal input_X6_Y0_b: unsigned(7 downto 0);
signal input_X6_Y1_r: unsigned(7 downto 0);
signal input_X6_Y1_g: unsigned(7 downto 0);
signal input_X6_Y1_b: unsigned(7 downto 0);
signal input_X6_Y2_r: unsigned(7 downto 0);
signal input_X6_Y2_g: unsigned(7 downto 0);
signal input_X6_Y2_b: unsigned(7 downto 0);
signal input_X6_Y3_r: unsigned(7 downto 0);
signal input_X6_Y3_g: unsigned(7 downto 0);
signal input_X6_Y3_b: unsigned(7 downto 0);
signal input_X6_Y4_r: unsigned(7 downto 0);
signal input_X6_Y4_g: unsigned(7 downto 0);
signal input_X6_Y4_b: unsigned(7 downto 0);
signal input_X6_Y5_r: unsigned(7 downto 0);
signal input_X6_Y5_g: unsigned(7 downto 0);
signal input_X6_Y5_b: unsigned(7 downto 0);
signal input_X6_Y6_r: unsigned(7 downto 0);
signal input_X6_Y6_g: unsigned(7 downto 0);
signal input_X6_Y6_b: unsigned(7 downto 0);
signal input_X6_Y7_r: unsigned(7 downto 0);
signal input_X6_Y7_g: unsigned(7 downto 0);
signal input_X6_Y7_b: unsigned(7 downto 0);
signal input_X7_Y0_r: unsigned(7 downto 0);
signal input_X7_Y0_g: unsigned(7 downto 0);
signal input_X7_Y0_b: unsigned(7 downto 0);
signal input_X7_Y1_r: unsigned(7 downto 0);
signal input_X7_Y1_g: unsigned(7 downto 0);
signal input_X7_Y1_b: unsigned(7 downto 0);
signal input_X7_Y2_r: unsigned(7 downto 0);
signal input_X7_Y2_g: unsigned(7 downto 0);
signal input_X7_Y2_b: unsigned(7 downto 0);
signal input_X7_Y3_r: unsigned(7 downto 0);
signal input_X7_Y3_g: unsigned(7 downto 0);
signal input_X7_Y3_b: unsigned(7 downto 0);
signal input_X7_Y4_r: unsigned(7 downto 0);
signal input_X7_Y4_g: unsigned(7 downto 0);
signal input_X7_Y4_b: unsigned(7 downto 0);
signal input_X7_Y5_r: unsigned(7 downto 0);
signal input_X7_Y5_g: unsigned(7 downto 0);
signal input_X7_Y5_b: unsigned(7 downto 0);
signal input_X7_Y6_r: unsigned(7 downto 0);
signal input_X7_Y6_g: unsigned(7 downto 0);
signal input_X7_Y6_b: unsigned(7 downto 0);
signal input_X7_Y7_r: unsigned(7 downto 0);
signal input_X7_Y7_g: unsigned(7 downto 0);
signal input_X7_Y7_b: unsigned(7 downto 0);


    --output
    signal output_X0_Y0_gray: unsigned(7 downto 0);
signal output_X0_Y1_gray: unsigned(7 downto 0);
signal output_X0_Y2_gray: unsigned(7 downto 0);
signal output_X0_Y3_gray: unsigned(7 downto 0);
signal output_X0_Y4_gray: unsigned(7 downto 0);
signal output_X0_Y5_gray: unsigned(7 downto 0);
signal output_X0_Y6_gray: unsigned(7 downto 0);
signal output_X0_Y7_gray: unsigned(7 downto 0);
signal output_X1_Y0_gray: unsigned(7 downto 0);
signal output_X1_Y1_gray: unsigned(7 downto 0);
signal output_X1_Y2_gray: unsigned(7 downto 0);
signal output_X1_Y3_gray: unsigned(7 downto 0);
signal output_X1_Y4_gray: unsigned(7 downto 0);
signal output_X1_Y5_gray: unsigned(7 downto 0);
signal output_X1_Y6_gray: unsigned(7 downto 0);
signal output_X1_Y7_gray: unsigned(7 downto 0);
signal output_X2_Y0_gray: unsigned(7 downto 0);
signal output_X2_Y1_gray: unsigned(7 downto 0);
signal output_X2_Y2_gray: unsigned(7 downto 0);
signal output_X2_Y3_gray: unsigned(7 downto 0);
signal output_X2_Y4_gray: unsigned(7 downto 0);
signal output_X2_Y5_gray: unsigned(7 downto 0);
signal output_X2_Y6_gray: unsigned(7 downto 0);
signal output_X2_Y7_gray: unsigned(7 downto 0);
signal output_X3_Y0_gray: unsigned(7 downto 0);
signal output_X3_Y1_gray: unsigned(7 downto 0);
signal output_X3_Y2_gray: unsigned(7 downto 0);
signal output_X3_Y3_gray: unsigned(7 downto 0);
signal output_X3_Y4_gray: unsigned(7 downto 0);
signal output_X3_Y5_gray: unsigned(7 downto 0);
signal output_X3_Y6_gray: unsigned(7 downto 0);
signal output_X3_Y7_gray: unsigned(7 downto 0);
signal output_X4_Y0_gray: unsigned(7 downto 0);
signal output_X4_Y1_gray: unsigned(7 downto 0);
signal output_X4_Y2_gray: unsigned(7 downto 0);
signal output_X4_Y3_gray: unsigned(7 downto 0);
signal output_X4_Y4_gray: unsigned(7 downto 0);
signal output_X4_Y5_gray: unsigned(7 downto 0);
signal output_X4_Y6_gray: unsigned(7 downto 0);
signal output_X4_Y7_gray: unsigned(7 downto 0);
signal output_X5_Y0_gray: unsigned(7 downto 0);
signal output_X5_Y1_gray: unsigned(7 downto 0);
signal output_X5_Y2_gray: unsigned(7 downto 0);
signal output_X5_Y3_gray: unsigned(7 downto 0);
signal output_X5_Y4_gray: unsigned(7 downto 0);
signal output_X5_Y5_gray: unsigned(7 downto 0);
signal output_X5_Y6_gray: unsigned(7 downto 0);
signal output_X5_Y7_gray: unsigned(7 downto 0);
signal output_X6_Y0_gray: unsigned(7 downto 0);
signal output_X6_Y1_gray: unsigned(7 downto 0);
signal output_X6_Y2_gray: unsigned(7 downto 0);
signal output_X6_Y3_gray: unsigned(7 downto 0);
signal output_X6_Y4_gray: unsigned(7 downto 0);
signal output_X6_Y5_gray: unsigned(7 downto 0);
signal output_X6_Y6_gray: unsigned(7 downto 0);
signal output_X6_Y7_gray: unsigned(7 downto 0);
signal output_X7_Y0_gray: unsigned(7 downto 0);
signal output_X7_Y1_gray: unsigned(7 downto 0);
signal output_X7_Y2_gray: unsigned(7 downto 0);
signal output_X7_Y3_gray: unsigned(7 downto 0);
signal output_X7_Y4_gray: unsigned(7 downto 0);
signal output_X7_Y5_gray: unsigned(7 downto 0);
signal output_X7_Y6_gray: unsigned(7 downto 0);
signal output_X7_Y7_gray: unsigned(7 downto 0);


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
    input_X0_Y0_r <= unsigned(data_in(7 downto 0));
input_X0_Y0_g <= unsigned(data_in(15 downto 8));
input_X0_Y0_b <= unsigned(data_in(23 downto 16));
input_X0_Y1_r <= unsigned(data_in(31 downto 24));
input_X0_Y1_g <= unsigned(data_in(39 downto 32));
input_X0_Y1_b <= unsigned(data_in(47 downto 40));
input_X0_Y2_r <= unsigned(data_in(55 downto 48));
input_X0_Y2_g <= unsigned(data_in(63 downto 56));
input_X0_Y2_b <= unsigned(data_in(71 downto 64));
input_X0_Y3_r <= unsigned(data_in(79 downto 72));
input_X0_Y3_g <= unsigned(data_in(87 downto 80));
input_X0_Y3_b <= unsigned(data_in(95 downto 88));
input_X0_Y4_r <= unsigned(data_in(103 downto 96));
input_X0_Y4_g <= unsigned(data_in(111 downto 104));
input_X0_Y4_b <= unsigned(data_in(119 downto 112));
input_X0_Y5_r <= unsigned(data_in(127 downto 120));
input_X0_Y5_g <= unsigned(data_in(135 downto 128));
input_X0_Y5_b <= unsigned(data_in(143 downto 136));
input_X0_Y6_r <= unsigned(data_in(151 downto 144));
input_X0_Y6_g <= unsigned(data_in(159 downto 152));
input_X0_Y6_b <= unsigned(data_in(167 downto 160));
input_X0_Y7_r <= unsigned(data_in(175 downto 168));
input_X0_Y7_g <= unsigned(data_in(183 downto 176));
input_X0_Y7_b <= unsigned(data_in(191 downto 184));
input_X1_Y0_r <= unsigned(data_in(199 downto 192));
input_X1_Y0_g <= unsigned(data_in(207 downto 200));
input_X1_Y0_b <= unsigned(data_in(215 downto 208));
input_X1_Y1_r <= unsigned(data_in(223 downto 216));
input_X1_Y1_g <= unsigned(data_in(231 downto 224));
input_X1_Y1_b <= unsigned(data_in(239 downto 232));
input_X1_Y2_r <= unsigned(data_in(247 downto 240));
input_X1_Y2_g <= unsigned(data_in(255 downto 248));
input_X1_Y2_b <= unsigned(data_in(263 downto 256));
input_X1_Y3_r <= unsigned(data_in(271 downto 264));
input_X1_Y3_g <= unsigned(data_in(279 downto 272));
input_X1_Y3_b <= unsigned(data_in(287 downto 280));
input_X1_Y4_r <= unsigned(data_in(295 downto 288));
input_X1_Y4_g <= unsigned(data_in(303 downto 296));
input_X1_Y4_b <= unsigned(data_in(311 downto 304));
input_X1_Y5_r <= unsigned(data_in(319 downto 312));
input_X1_Y5_g <= unsigned(data_in(327 downto 320));
input_X1_Y5_b <= unsigned(data_in(335 downto 328));
input_X1_Y6_r <= unsigned(data_in(343 downto 336));
input_X1_Y6_g <= unsigned(data_in(351 downto 344));
input_X1_Y6_b <= unsigned(data_in(359 downto 352));
input_X1_Y7_r <= unsigned(data_in(367 downto 360));
input_X1_Y7_g <= unsigned(data_in(375 downto 368));
input_X1_Y7_b <= unsigned(data_in(383 downto 376));
input_X2_Y0_r <= unsigned(data_in(391 downto 384));
input_X2_Y0_g <= unsigned(data_in(399 downto 392));
input_X2_Y0_b <= unsigned(data_in(407 downto 400));
input_X2_Y1_r <= unsigned(data_in(415 downto 408));
input_X2_Y1_g <= unsigned(data_in(423 downto 416));
input_X2_Y1_b <= unsigned(data_in(431 downto 424));
input_X2_Y2_r <= unsigned(data_in(439 downto 432));
input_X2_Y2_g <= unsigned(data_in(447 downto 440));
input_X2_Y2_b <= unsigned(data_in(455 downto 448));
input_X2_Y3_r <= unsigned(data_in(463 downto 456));
input_X2_Y3_g <= unsigned(data_in(471 downto 464));
input_X2_Y3_b <= unsigned(data_in(479 downto 472));
input_X2_Y4_r <= unsigned(data_in(487 downto 480));
input_X2_Y4_g <= unsigned(data_in(495 downto 488));
input_X2_Y4_b <= unsigned(data_in(503 downto 496));
input_X2_Y5_r <= unsigned(data_in(511 downto 504));
input_X2_Y5_g <= unsigned(data_in(519 downto 512));
input_X2_Y5_b <= unsigned(data_in(527 downto 520));
input_X2_Y6_r <= unsigned(data_in(535 downto 528));
input_X2_Y6_g <= unsigned(data_in(543 downto 536));
input_X2_Y6_b <= unsigned(data_in(551 downto 544));
input_X2_Y7_r <= unsigned(data_in(559 downto 552));
input_X2_Y7_g <= unsigned(data_in(567 downto 560));
input_X2_Y7_b <= unsigned(data_in(575 downto 568));
input_X3_Y0_r <= unsigned(data_in(583 downto 576));
input_X3_Y0_g <= unsigned(data_in(591 downto 584));
input_X3_Y0_b <= unsigned(data_in(599 downto 592));
input_X3_Y1_r <= unsigned(data_in(607 downto 600));
input_X3_Y1_g <= unsigned(data_in(615 downto 608));
input_X3_Y1_b <= unsigned(data_in(623 downto 616));
input_X3_Y2_r <= unsigned(data_in(631 downto 624));
input_X3_Y2_g <= unsigned(data_in(639 downto 632));
input_X3_Y2_b <= unsigned(data_in(647 downto 640));
input_X3_Y3_r <= unsigned(data_in(655 downto 648));
input_X3_Y3_g <= unsigned(data_in(663 downto 656));
input_X3_Y3_b <= unsigned(data_in(671 downto 664));
input_X3_Y4_r <= unsigned(data_in(679 downto 672));
input_X3_Y4_g <= unsigned(data_in(687 downto 680));
input_X3_Y4_b <= unsigned(data_in(695 downto 688));
input_X3_Y5_r <= unsigned(data_in(703 downto 696));
input_X3_Y5_g <= unsigned(data_in(711 downto 704));
input_X3_Y5_b <= unsigned(data_in(719 downto 712));
input_X3_Y6_r <= unsigned(data_in(727 downto 720));
input_X3_Y6_g <= unsigned(data_in(735 downto 728));
input_X3_Y6_b <= unsigned(data_in(743 downto 736));
input_X3_Y7_r <= unsigned(data_in(751 downto 744));
input_X3_Y7_g <= unsigned(data_in(759 downto 752));
input_X3_Y7_b <= unsigned(data_in(767 downto 760));
input_X4_Y0_r <= unsigned(data_in(775 downto 768));
input_X4_Y0_g <= unsigned(data_in(783 downto 776));
input_X4_Y0_b <= unsigned(data_in(791 downto 784));
input_X4_Y1_r <= unsigned(data_in(799 downto 792));
input_X4_Y1_g <= unsigned(data_in(807 downto 800));
input_X4_Y1_b <= unsigned(data_in(815 downto 808));
input_X4_Y2_r <= unsigned(data_in(823 downto 816));
input_X4_Y2_g <= unsigned(data_in(831 downto 824));
input_X4_Y2_b <= unsigned(data_in(839 downto 832));
input_X4_Y3_r <= unsigned(data_in(847 downto 840));
input_X4_Y3_g <= unsigned(data_in(855 downto 848));
input_X4_Y3_b <= unsigned(data_in(863 downto 856));
input_X4_Y4_r <= unsigned(data_in(871 downto 864));
input_X4_Y4_g <= unsigned(data_in(879 downto 872));
input_X4_Y4_b <= unsigned(data_in(887 downto 880));
input_X4_Y5_r <= unsigned(data_in(895 downto 888));
input_X4_Y5_g <= unsigned(data_in(903 downto 896));
input_X4_Y5_b <= unsigned(data_in(911 downto 904));
input_X4_Y6_r <= unsigned(data_in(919 downto 912));
input_X4_Y6_g <= unsigned(data_in(927 downto 920));
input_X4_Y6_b <= unsigned(data_in(935 downto 928));
input_X4_Y7_r <= unsigned(data_in(943 downto 936));
input_X4_Y7_g <= unsigned(data_in(951 downto 944));
input_X4_Y7_b <= unsigned(data_in(959 downto 952));
input_X5_Y0_r <= unsigned(data_in(967 downto 960));
input_X5_Y0_g <= unsigned(data_in(975 downto 968));
input_X5_Y0_b <= unsigned(data_in(983 downto 976));
input_X5_Y1_r <= unsigned(data_in(991 downto 984));
input_X5_Y1_g <= unsigned(data_in(999 downto 992));
input_X5_Y1_b <= unsigned(data_in(1007 downto 1000));
input_X5_Y2_r <= unsigned(data_in(1015 downto 1008));
input_X5_Y2_g <= unsigned(data_in(1023 downto 1016));
input_X5_Y2_b <= unsigned(data_in(1031 downto 1024));
input_X5_Y3_r <= unsigned(data_in(1039 downto 1032));
input_X5_Y3_g <= unsigned(data_in(1047 downto 1040));
input_X5_Y3_b <= unsigned(data_in(1055 downto 1048));
input_X5_Y4_r <= unsigned(data_in(1063 downto 1056));
input_X5_Y4_g <= unsigned(data_in(1071 downto 1064));
input_X5_Y4_b <= unsigned(data_in(1079 downto 1072));
input_X5_Y5_r <= unsigned(data_in(1087 downto 1080));
input_X5_Y5_g <= unsigned(data_in(1095 downto 1088));
input_X5_Y5_b <= unsigned(data_in(1103 downto 1096));
input_X5_Y6_r <= unsigned(data_in(1111 downto 1104));
input_X5_Y6_g <= unsigned(data_in(1119 downto 1112));
input_X5_Y6_b <= unsigned(data_in(1127 downto 1120));
input_X5_Y7_r <= unsigned(data_in(1135 downto 1128));
input_X5_Y7_g <= unsigned(data_in(1143 downto 1136));
input_X5_Y7_b <= unsigned(data_in(1151 downto 1144));
input_X6_Y0_r <= unsigned(data_in(1159 downto 1152));
input_X6_Y0_g <= unsigned(data_in(1167 downto 1160));
input_X6_Y0_b <= unsigned(data_in(1175 downto 1168));
input_X6_Y1_r <= unsigned(data_in(1183 downto 1176));
input_X6_Y1_g <= unsigned(data_in(1191 downto 1184));
input_X6_Y1_b <= unsigned(data_in(1199 downto 1192));
input_X6_Y2_r <= unsigned(data_in(1207 downto 1200));
input_X6_Y2_g <= unsigned(data_in(1215 downto 1208));
input_X6_Y2_b <= unsigned(data_in(1223 downto 1216));
input_X6_Y3_r <= unsigned(data_in(1231 downto 1224));
input_X6_Y3_g <= unsigned(data_in(1239 downto 1232));
input_X6_Y3_b <= unsigned(data_in(1247 downto 1240));
input_X6_Y4_r <= unsigned(data_in(1255 downto 1248));
input_X6_Y4_g <= unsigned(data_in(1263 downto 1256));
input_X6_Y4_b <= unsigned(data_in(1271 downto 1264));
input_X6_Y5_r <= unsigned(data_in(1279 downto 1272));
input_X6_Y5_g <= unsigned(data_in(1287 downto 1280));
input_X6_Y5_b <= unsigned(data_in(1295 downto 1288));
input_X6_Y6_r <= unsigned(data_in(1303 downto 1296));
input_X6_Y6_g <= unsigned(data_in(1311 downto 1304));
input_X6_Y6_b <= unsigned(data_in(1319 downto 1312));
input_X6_Y7_r <= unsigned(data_in(1327 downto 1320));
input_X6_Y7_g <= unsigned(data_in(1335 downto 1328));
input_X6_Y7_b <= unsigned(data_in(1343 downto 1336));
input_X7_Y0_r <= unsigned(data_in(1351 downto 1344));
input_X7_Y0_g <= unsigned(data_in(1359 downto 1352));
input_X7_Y0_b <= unsigned(data_in(1367 downto 1360));
input_X7_Y1_r <= unsigned(data_in(1375 downto 1368));
input_X7_Y1_g <= unsigned(data_in(1383 downto 1376));
input_X7_Y1_b <= unsigned(data_in(1391 downto 1384));
input_X7_Y2_r <= unsigned(data_in(1399 downto 1392));
input_X7_Y2_g <= unsigned(data_in(1407 downto 1400));
input_X7_Y2_b <= unsigned(data_in(1415 downto 1408));
input_X7_Y3_r <= unsigned(data_in(1423 downto 1416));
input_X7_Y3_g <= unsigned(data_in(1431 downto 1424));
input_X7_Y3_b <= unsigned(data_in(1439 downto 1432));
input_X7_Y4_r <= unsigned(data_in(1447 downto 1440));
input_X7_Y4_g <= unsigned(data_in(1455 downto 1448));
input_X7_Y4_b <= unsigned(data_in(1463 downto 1456));
input_X7_Y5_r <= unsigned(data_in(1471 downto 1464));
input_X7_Y5_g <= unsigned(data_in(1479 downto 1472));
input_X7_Y5_b <= unsigned(data_in(1487 downto 1480));
input_X7_Y6_r <= unsigned(data_in(1495 downto 1488));
input_X7_Y6_g <= unsigned(data_in(1503 downto 1496));
input_X7_Y6_b <= unsigned(data_in(1511 downto 1504));
input_X7_Y7_r <= unsigned(data_in(1519 downto 1512));
input_X7_Y7_g <= unsigned(data_in(1527 downto 1520));
input_X7_Y7_b <= unsigned(data_in(1535 downto 1528));


    --deconstruction outputs
    data_out(7 downto 0) <= std_logic_vector(output_X0_Y0_gray);
data_out(15 downto 8) <= std_logic_vector(output_X0_Y1_gray);
data_out(23 downto 16) <= std_logic_vector(output_X0_Y2_gray);
data_out(31 downto 24) <= std_logic_vector(output_X0_Y3_gray);
data_out(39 downto 32) <= std_logic_vector(output_X0_Y4_gray);
data_out(47 downto 40) <= std_logic_vector(output_X0_Y5_gray);
data_out(55 downto 48) <= std_logic_vector(output_X0_Y6_gray);
data_out(63 downto 56) <= std_logic_vector(output_X0_Y7_gray);
data_out(71 downto 64) <= std_logic_vector(output_X1_Y0_gray);
data_out(79 downto 72) <= std_logic_vector(output_X1_Y1_gray);
data_out(87 downto 80) <= std_logic_vector(output_X1_Y2_gray);
data_out(95 downto 88) <= std_logic_vector(output_X1_Y3_gray);
data_out(103 downto 96) <= std_logic_vector(output_X1_Y4_gray);
data_out(111 downto 104) <= std_logic_vector(output_X1_Y5_gray);
data_out(119 downto 112) <= std_logic_vector(output_X1_Y6_gray);
data_out(127 downto 120) <= std_logic_vector(output_X1_Y7_gray);
data_out(135 downto 128) <= std_logic_vector(output_X2_Y0_gray);
data_out(143 downto 136) <= std_logic_vector(output_X2_Y1_gray);
data_out(151 downto 144) <= std_logic_vector(output_X2_Y2_gray);
data_out(159 downto 152) <= std_logic_vector(output_X2_Y3_gray);
data_out(167 downto 160) <= std_logic_vector(output_X2_Y4_gray);
data_out(175 downto 168) <= std_logic_vector(output_X2_Y5_gray);
data_out(183 downto 176) <= std_logic_vector(output_X2_Y6_gray);
data_out(191 downto 184) <= std_logic_vector(output_X2_Y7_gray);
data_out(199 downto 192) <= std_logic_vector(output_X3_Y0_gray);
data_out(207 downto 200) <= std_logic_vector(output_X3_Y1_gray);
data_out(215 downto 208) <= std_logic_vector(output_X3_Y2_gray);
data_out(223 downto 216) <= std_logic_vector(output_X3_Y3_gray);
data_out(231 downto 224) <= std_logic_vector(output_X3_Y4_gray);
data_out(239 downto 232) <= std_logic_vector(output_X3_Y5_gray);
data_out(247 downto 240) <= std_logic_vector(output_X3_Y6_gray);
data_out(255 downto 248) <= std_logic_vector(output_X3_Y7_gray);
data_out(263 downto 256) <= std_logic_vector(output_X4_Y0_gray);
data_out(271 downto 264) <= std_logic_vector(output_X4_Y1_gray);
data_out(279 downto 272) <= std_logic_vector(output_X4_Y2_gray);
data_out(287 downto 280) <= std_logic_vector(output_X4_Y3_gray);
data_out(295 downto 288) <= std_logic_vector(output_X4_Y4_gray);
data_out(303 downto 296) <= std_logic_vector(output_X4_Y5_gray);
data_out(311 downto 304) <= std_logic_vector(output_X4_Y6_gray);
data_out(319 downto 312) <= std_logic_vector(output_X4_Y7_gray);
data_out(327 downto 320) <= std_logic_vector(output_X5_Y0_gray);
data_out(335 downto 328) <= std_logic_vector(output_X5_Y1_gray);
data_out(343 downto 336) <= std_logic_vector(output_X5_Y2_gray);
data_out(351 downto 344) <= std_logic_vector(output_X5_Y3_gray);
data_out(359 downto 352) <= std_logic_vector(output_X5_Y4_gray);
data_out(367 downto 360) <= std_logic_vector(output_X5_Y5_gray);
data_out(375 downto 368) <= std_logic_vector(output_X5_Y6_gray);
data_out(383 downto 376) <= std_logic_vector(output_X5_Y7_gray);
data_out(391 downto 384) <= std_logic_vector(output_X6_Y0_gray);
data_out(399 downto 392) <= std_logic_vector(output_X6_Y1_gray);
data_out(407 downto 400) <= std_logic_vector(output_X6_Y2_gray);
data_out(415 downto 408) <= std_logic_vector(output_X6_Y3_gray);
data_out(423 downto 416) <= std_logic_vector(output_X6_Y4_gray);
data_out(431 downto 424) <= std_logic_vector(output_X6_Y5_gray);
data_out(439 downto 432) <= std_logic_vector(output_X6_Y6_gray);
data_out(447 downto 440) <= std_logic_vector(output_X6_Y7_gray);
data_out(455 downto 448) <= std_logic_vector(output_X7_Y0_gray);
data_out(463 downto 456) <= std_logic_vector(output_X7_Y1_gray);
data_out(471 downto 464) <= std_logic_vector(output_X7_Y2_gray);
data_out(479 downto 472) <= std_logic_vector(output_X7_Y3_gray);
data_out(487 downto 480) <= std_logic_vector(output_X7_Y4_gray);
data_out(495 downto 488) <= std_logic_vector(output_X7_Y5_gray);
data_out(503 downto 496) <= std_logic_vector(output_X7_Y6_gray);
data_out(511 downto 504) <= std_logic_vector(output_X7_Y7_gray);


    --
    --
    --      INSERT HERE YOUR VHDL CODE
    --
    --
    --


end Behavioral;


